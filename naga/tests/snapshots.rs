// A lot of the code can be unused based on configuration flags,
// the corresponding warnings aren't helpful.
#![allow(dead_code, unused_imports)]

use std::{
    fs,
    path::{Path, PathBuf},
};

const CRATE_ROOT: &str = env!("CARGO_MANIFEST_DIR");
const BASE_DIR_IN: &str = "tests/in";
const BASE_DIR_OUT: &str = "tests/out";

bitflags::bitflags! {
    #[derive(Clone, Copy)]
    struct Targets: u32 {
        /// A serialization of the `naga::Module`, in RON format.
        const IR = 1;

        /// A serialization of the `naga::valid::ModuleInfo`, in RON format.
        const ANALYSIS = 1 << 1;

        const SPIRV = 1 << 2;
        const METAL = 1 << 3;
        const GLSL = 1 << 4;
        const DOT = 1 << 5;
        const HLSL = 1 << 6;
        const WGSL = 1 << 7;
        const NO_VALIDATION = 1 << 8;
    }
}

#[derive(serde::Deserialize)]
struct SpvOutVersion(u8, u8);
impl Default for SpvOutVersion {
    fn default() -> Self {
        SpvOutVersion(1, 1)
    }
}

#[derive(Default, serde::Deserialize)]
#[serde(default)]
struct SpirvOutParameters {
    version: SpvOutVersion,
    capabilities: naga::FastHashSet<spirv::Capability>,
    debug: bool,
    adjust_coordinate_space: bool,
    force_point_size: bool,
    clamp_frag_depth: bool,
    separate_entry_points: bool,
    #[cfg(all(feature = "deserialize", spv_out))]
    binding_map: naga::back::spv::BindingMap,
}

#[derive(Default, serde::Deserialize)]
#[serde(default)]
struct WgslOutParameters {
    explicit_types: bool,
}

#[derive(Default, serde::Deserialize)]
#[serde(default)]
struct Parameters {
    // -- GOD MODE --
    god_mode: bool,

    // -- SPIR-V options --
    spv: SpirvOutParameters,

    // -- MSL options --
    #[cfg(all(feature = "deserialize", msl_out))]
    msl: naga::back::msl::Options,
    #[cfg(all(feature = "deserialize", msl_out))]
    #[serde(default)]
    msl_pipeline: naga::back::msl::PipelineOptions,

    // -- GLSL options --
    #[cfg(all(feature = "deserialize", glsl_out))]
    glsl: naga::back::glsl::Options,
    glsl_exclude_list: naga::FastHashSet<String>,
    #[cfg(all(feature = "deserialize", glsl_out))]
    glsl_multiview: Option<std::num::NonZeroU32>,

    // -- HLSL options --
    #[cfg(all(feature = "deserialize", hlsl_out))]
    hlsl: naga::back::hlsl::Options,

    // -- WGSL options --
    wgsl: WgslOutParameters,

    // -- General options --
    #[cfg(feature = "deserialize")]
    bounds_check_policies: naga::proc::BoundsCheckPolicies,

    #[cfg(all(feature = "deserialize", any(hlsl_out, msl_out, spv_out, glsl_out)))]
    pipeline_constants: naga::back::PipelineConstants,
}

/// Information about a shader input file.
#[derive(Debug)]
struct Input {
    /// The subdirectory of `tests/in` to which this input belongs, if any.
    ///
    /// If the subdirectory is omitted, we assume that the output goes
    /// to "wgsl".
    subdirectory: Option<PathBuf>,

    /// The input filename name, without a directory.
    file_name: PathBuf,

    /// True if output filenames should add the output extension on top of
    /// `file_name`'s existing extension, rather than replacing it.
    ///
    /// This is used by `convert_glsl_folder`, which wants to take input files
    /// like `210-bevy-2d-shader.frag` and just add `.wgsl` to it, producing
    /// `210-bevy-2d-shader.frag.wgsl`.
    keep_input_extension: bool,
}

impl Input {
    /// Read an input file and its corresponding parameters file.
    ///
    /// Given `input`, the relative path of a shader input file, return
    /// a `Source` value containing its path, code, and parameters.
    ///
    /// The `input` path is interpreted relative to the `BASE_DIR_IN`
    /// subdirectory of the directory given by the `CARGO_MANIFEST_DIR`
    /// environment variable.
    fn new(subdirectory: Option<&str>, name: &str, extension: &str) -> Input {
        Input {
            subdirectory: subdirectory.map(PathBuf::from),
            // Don't wipe out any extensions on `name`, as
            // `with_extension` would do.
            file_name: PathBuf::from(format!("{name}.{extension}")),
            keep_input_extension: false,
        }
    }

    /// Return an iterator that produces an `Input` for each entry in `subdirectory`.
    fn files_in_dir(subdirectory: &str) -> impl Iterator<Item = Input> + 'static {
        let subdirectory = subdirectory.to_string();
        let mut input_directory = Path::new(env!("CARGO_MANIFEST_DIR")).join(BASE_DIR_IN);
        input_directory.push(&subdirectory);
        match std::fs::read_dir(&input_directory) {
            Ok(entries) => entries.map(move |result| {
                let entry = result.expect("error reading directory");
                let file_name = PathBuf::from(entry.file_name());
                let extension = file_name
                    .extension()
                    .expect("all files in snapshot input directory should have extensions");
                let input = Input::new(
                    Some(&subdirectory),
                    file_name.file_stem().unwrap().to_str().unwrap(),
                    extension.to_str().unwrap(),
                );
                input
            }),
            Err(err) => {
                panic!(
                    "Error opening directory '{}': {}",
                    input_directory.display(),
                    err
                );
            }
        }
    }

    /// Return the path to the input directory.
    fn input_directory(&self) -> PathBuf {
        let mut dir = Path::new(CRATE_ROOT).join(BASE_DIR_IN);
        if let Some(ref subdirectory) = self.subdirectory {
            dir.push(subdirectory);
        }
        dir
    }

    /// Return the path to the output directory.
    fn output_directory(&self, subdirectory: &str) -> PathBuf {
        let mut dir = Path::new(CRATE_ROOT).join(BASE_DIR_OUT);
        dir.push(subdirectory);
        dir
    }

    /// Return the path to the input file.
    fn input_path(&self) -> PathBuf {
        let mut input = self.input_directory();
        input.push(&self.file_name);
        input
    }

    fn output_path(&self, subdirectory: &str, extension: &str) -> PathBuf {
        let mut output = self.output_directory(subdirectory);
        if self.keep_input_extension {
            let mut file_name = self.file_name.as_os_str().to_owned();
            file_name.push(".");
            file_name.push(extension);
            output.push(&file_name);
        } else {
            output.push(&self.file_name);
            output.set_extension(extension);
        }
        output
    }

    /// Return the contents of the input file as a string.
    fn read_source(&self) -> String {
        println!("Processing '{}'", self.file_name.display());
        let input_path = self.input_path();
        match fs::read_to_string(&input_path) {
            Ok(source) => source,
            Err(err) => {
                panic!(
                    "Couldn't read shader input file `{}`: {}",
                    input_path.display(),
                    err
                );
            }
        }
    }

    /// Return the contents of the input file as a vector of bytes.
    fn read_bytes(&self) -> Vec<u8> {
        println!("Processing '{}'", self.file_name.display());
        let input_path = self.input_path();
        match fs::read(&input_path) {
            Ok(bytes) => bytes,
            Err(err) => {
                panic!(
                    "Couldn't read shader input file `{}`: {}",
                    input_path.display(),
                    err
                );
            }
        }
    }

    /// Return this input's parameter file, parsed.
    fn read_parameters(&self) -> Parameters {
        let mut param_path = self.input_path();
        param_path.set_extension("param.ron");
        match fs::read_to_string(&param_path) {
            Ok(string) => match ron::de::from_str(&string) {
                Ok(params) => params,
                Err(e) => panic!(
                    "Couldn't parse param file: {} due to: {e}",
                    param_path.display()
                ),
            },
            Err(_) => Parameters::default(),
        }
    }

    /// Write `data` to a file corresponding to this input file in
    /// `subdirectory`, with `extension`.
    fn write_output_file(&self, subdirectory: &str, extension: &str, data: impl AsRef<[u8]>) {
        let output_path = self.output_path(subdirectory, extension);
        if let Err(err) = fs::write(&output_path, data) {
            panic!("Error writing {}: {}", output_path.display(), err);
        }
    }
}

#[cfg(hlsl_out)]
type FragmentEntryPoint<'a> = naga::back::hlsl::FragmentEntryPoint<'a>;
#[cfg(not(hlsl_out))]
type FragmentEntryPoint<'a> = ();

#[allow(unused_variables)]
fn check_targets(
    input: &Input,
    module: &mut naga::Module,
    targets: Targets,
    source_code: Option<&str>,
    // For testing hlsl generation when fragment shader doesn't consume all vertex outputs.
    frag_ep: Option<FragmentEntryPoint>,
) {
    if frag_ep.is_some() && !targets.contains(Targets::HLSL) {
        panic!("Providing FragmentEntryPoint only makes sense when testing hlsl-out");
    }

    let params = input.read_parameters();
    let name = &input.file_name;

    let (capabilities, subgroup_stages, subgroup_operations) = if params.god_mode {
        (
            naga::valid::Capabilities::all(),
            naga::valid::ShaderStages::all(),
            naga::valid::SubgroupOperationSet::all(),
        )
    } else {
        (
            naga::valid::Capabilities::default(),
            naga::valid::ShaderStages::empty(),
            naga::valid::SubgroupOperationSet::empty(),
        )
    };

    #[cfg(feature = "serialize")]
    {
        if targets.contains(Targets::IR) {
            let config = ron::ser::PrettyConfig::default().new_line("\n".to_string());
            let string = ron::ser::to_string_pretty(module, config).unwrap();
            input.write_output_file("ir", "ron", string);
        }
    }

    let validation_flags = if targets.contains(Targets::NO_VALIDATION) {
        naga::valid::ValidationFlags::empty()
    } else {
        naga::valid::ValidationFlags::all()
    };

    let info = naga::valid::Validator::new(validation_flags, capabilities)
        .subgroup_stages(subgroup_stages)
        .subgroup_operations(subgroup_operations)
        .validate(module)
        .unwrap_or_else(|err| {
            panic!(
                "Naga module validation failed on test `{}`:\n{:?}",
                name.display(),
                err
            );
        });

    #[cfg(feature = "compact")]
    let info = {
        naga::compact::compact(module);

        #[cfg(feature = "serialize")]
        {
            if targets.contains(Targets::IR) {
                let config = ron::ser::PrettyConfig::default().new_line("\n".to_string());
                let string = ron::ser::to_string_pretty(module, config).unwrap();
                input.write_output_file("ir", "compact.ron", string);
            }
        }

        naga::valid::Validator::new(validation_flags, capabilities)
            .subgroup_stages(subgroup_stages)
            .subgroup_operations(subgroup_operations)
            .validate(module)
            .unwrap_or_else(|err| {
                panic!(
                    "Post-compaction module validation failed on test '{}':\n<{:?}",
                    name.display(),
                    err,
                )
            })
    };

    #[cfg(feature = "serialize")]
    {
        if targets.contains(Targets::ANALYSIS) {
            let config = ron::ser::PrettyConfig::default().new_line("\n".to_string());
            let string = ron::ser::to_string_pretty(&info, config).unwrap();
            input.write_output_file("analysis", "info.ron", string);
        }
    }

    #[cfg(all(feature = "deserialize", spv_out))]
    {
        let debug_info = source_code.map(|code| naga::back::spv::DebugInfo {
            source_code: code,
            file_name: name.as_ref(),
            // wgpu#6266: we technically know all the information here to
            // produce the valid language but it's not too important for
            // validation purposes
            language: naga::back::spv::SourceLanguage::Unknown,
        });

        if targets.contains(Targets::SPIRV) {
            write_output_spv(
                input,
                module,
                &info,
                debug_info,
                &params.spv,
                params.bounds_check_policies,
                &params.pipeline_constants,
            );
        }
    }
    #[cfg(all(feature = "deserialize", msl_out))]
    {
        if targets.contains(Targets::METAL) {
            write_output_msl(
                input,
                module,
                &info,
                &params.msl,
                &params.msl_pipeline,
                params.bounds_check_policies,
                &params.pipeline_constants,
            );
        }
    }
    #[cfg(all(feature = "deserialize", glsl_out))]
    {
        if targets.contains(Targets::GLSL) {
            for ep in module.entry_points.iter() {
                if params.glsl_exclude_list.contains(&ep.name) {
                    continue;
                }
                write_output_glsl(
                    input,
                    module,
                    &info,
                    ep.stage,
                    &ep.name,
                    &params.glsl,
                    params.bounds_check_policies,
                    params.glsl_multiview,
                    &params.pipeline_constants,
                );
            }
        }
    }
    #[cfg(dot_out)]
    {
        if targets.contains(Targets::DOT) {
            let string = naga::back::dot::write(module, Some(&info), Default::default()).unwrap();
            input.write_output_file("dot", "dot", string);
        }
    }
    #[cfg(all(feature = "deserialize", hlsl_out))]
    {
        if targets.contains(Targets::HLSL) {
            write_output_hlsl(
                input,
                module,
                &info,
                &params.hlsl,
                &params.pipeline_constants,
                frag_ep,
            );
        }
    }
    #[cfg(all(feature = "deserialize", wgsl_out))]
    {
        if targets.contains(Targets::WGSL) {
            write_output_wgsl(input, module, &info, &params.wgsl);
        }
    }
}

#[cfg(spv_out)]
fn write_output_spv(
    input: &Input,
    module: &naga::Module,
    info: &naga::valid::ModuleInfo,
    debug_info: Option<naga::back::spv::DebugInfo>,
    params: &SpirvOutParameters,
    bounds_check_policies: naga::proc::BoundsCheckPolicies,
    pipeline_constants: &naga::back::PipelineConstants,
) {
    use naga::back::spv;
    use rspirv::binary::Disassemble;

    let mut flags = spv::WriterFlags::LABEL_VARYINGS;
    flags.set(spv::WriterFlags::DEBUG, params.debug);
    flags.set(
        spv::WriterFlags::ADJUST_COORDINATE_SPACE,
        params.adjust_coordinate_space,
    );
    flags.set(spv::WriterFlags::FORCE_POINT_SIZE, params.force_point_size);
    flags.set(spv::WriterFlags::CLAMP_FRAG_DEPTH, params.clamp_frag_depth);

    let options = spv::Options {
        lang_version: (params.version.0, params.version.1),
        flags,
        capabilities: if params.capabilities.is_empty() {
            None
        } else {
            Some(params.capabilities.clone())
        },
        bounds_check_policies,
        binding_map: params.binding_map.clone(),
        zero_initialize_workgroup_memory: spv::ZeroInitializeWorkgroupMemoryMode::Polyfill,
        debug_info,
    };

    let (module, info) =
        naga::back::pipeline_constants::process_overrides(module, info, pipeline_constants)
            .expect("override evaluation failed");

    if params.separate_entry_points {
        for ep in module.entry_points.iter() {
            let pipeline_options = spv::PipelineOptions {
                entry_point: ep.name.clone(),
                shader_stage: ep.stage,
            };
            write_output_spv_inner(
                input,
                &module,
                &info,
                &options,
                Some(&pipeline_options),
                &format!("{}.spvasm", ep.name),
            );
        }
    } else {
        write_output_spv_inner(input, &module, &info, &options, None, "spvasm");
    }
}

#[cfg(spv_out)]
fn write_output_spv_inner(
    input: &Input,
    module: &naga::Module,
    info: &naga::valid::ModuleInfo,
    options: &naga::back::spv::Options<'_>,
    pipeline_options: Option<&naga::back::spv::PipelineOptions>,
    extension: &str,
) {
    use naga::back::spv;
    use rspirv::binary::Disassemble;
    println!("Generating SPIR-V for {:?}", input.file_name);
    let spv = spv::write_vec(module, info, options, pipeline_options).unwrap();
    let dis = rspirv::dr::load_words(spv)
        .expect("Produced invalid SPIR-V")
        .disassemble();
    // HACK escape CR/LF if source code is in side.
    let dis = if options.debug_info.is_some() {
        let dis = dis.replace("\\r", "\r");
        dis.replace("\\n", "\n")
    } else {
        dis
    };
    input.write_output_file("spv", extension, dis);
}

#[cfg(msl_out)]
fn write_output_msl(
    input: &Input,
    module: &naga::Module,
    info: &naga::valid::ModuleInfo,
    options: &naga::back::msl::Options,
    pipeline_options: &naga::back::msl::PipelineOptions,
    bounds_check_policies: naga::proc::BoundsCheckPolicies,
    pipeline_constants: &naga::back::PipelineConstants,
) {
    use naga::back::msl;

    println!("generating MSL");

    let (module, info) =
        naga::back::pipeline_constants::process_overrides(module, info, pipeline_constants)
            .expect("override evaluation failed");

    let mut options = options.clone();
    options.bounds_check_policies = bounds_check_policies;
    let (string, tr_info) = msl::write_string(&module, &info, &options, pipeline_options)
        .unwrap_or_else(|err| panic!("Metal write failed: {err}"));

    for (ep, result) in module.entry_points.iter().zip(tr_info.entry_point_names) {
        if let Err(error) = result {
            panic!("Failed to translate '{}': {}", ep.name, error);
        }
    }

    input.write_output_file("msl", "msl", string);
}

#[cfg(glsl_out)]
#[allow(clippy::too_many_arguments)]
fn write_output_glsl(
    input: &Input,
    module: &naga::Module,
    info: &naga::valid::ModuleInfo,
    stage: naga::ShaderStage,
    ep_name: &str,
    options: &naga::back::glsl::Options,
    bounds_check_policies: naga::proc::BoundsCheckPolicies,
    multiview: Option<std::num::NonZeroU32>,
    pipeline_constants: &naga::back::PipelineConstants,
) {
    use naga::back::glsl;

    println!("generating GLSL");

    let pipeline_options = glsl::PipelineOptions {
        shader_stage: stage,
        entry_point: ep_name.to_string(),
        multiview,
    };

    let mut buffer = String::new();
    let (module, info) =
        naga::back::pipeline_constants::process_overrides(module, info, pipeline_constants)
            .expect("override evaluation failed");
    let mut writer = glsl::Writer::new(
        &mut buffer,
        &module,
        &info,
        options,
        &pipeline_options,
        bounds_check_policies,
    )
    .expect("GLSL init failed");
    writer.write().expect("GLSL write failed");

    let extension = format!("{ep_name}.{stage:?}.glsl");
    input.write_output_file("glsl", &extension, buffer);
}

#[cfg(hlsl_out)]
fn write_output_hlsl(
    input: &Input,
    module: &naga::Module,
    info: &naga::valid::ModuleInfo,
    options: &naga::back::hlsl::Options,
    pipeline_constants: &naga::back::PipelineConstants,
    frag_ep: Option<naga::back::hlsl::FragmentEntryPoint>,
) {
    use naga::back::hlsl;
    use std::fmt::Write as _;

    println!("generating HLSL");

    let (module, info) =
        naga::back::pipeline_constants::process_overrides(module, info, pipeline_constants)
            .expect("override evaluation failed");

    let mut buffer = String::new();
    let mut writer = hlsl::Writer::new(&mut buffer, options);
    let reflection_info = writer
        .write(&module, &info, frag_ep.as_ref())
        .expect("HLSL write failed");

    input.write_output_file("hlsl", "hlsl", buffer);

    // We need a config file for validation script
    // This file contains an info about profiles (shader stages) contains inside generated shader
    // This info will be passed to dxc
    let mut config = hlsl_snapshots::Config::empty();
    for (index, ep) in module.entry_points.iter().enumerate() {
        let name = match reflection_info.entry_point_names[index] {
            Ok(ref name) => name,
            Err(_) => continue,
        };
        match ep.stage {
            naga::ShaderStage::Vertex => &mut config.vertex,
            naga::ShaderStage::Fragment => &mut config.fragment,
            naga::ShaderStage::Compute => &mut config.compute,
        }
        .push(hlsl_snapshots::ConfigItem {
            entry_point: name.clone(),
            target_profile: format!(
                "{}_{}",
                ep.stage.to_hlsl_str(),
                options.shader_model.to_str()
            ),
        });
    }

    config.to_file(input.output_path("hlsl", "ron")).unwrap();
}

#[cfg(wgsl_out)]
fn write_output_wgsl(
    input: &Input,
    module: &naga::Module,
    info: &naga::valid::ModuleInfo,
    params: &WgslOutParameters,
) {
    use naga::back::wgsl;

    println!("generating WGSL");

    let mut flags = wgsl::WriterFlags::empty();
    flags.set(wgsl::WriterFlags::EXPLICIT_TYPES, params.explicit_types);

    let string = wgsl::write_string(module, info, flags).expect("WGSL write failed");

    input.write_output_file("wgsl", "wgsl", string);
}

#[cfg(feature = "wgsl-in")]
#[test]
fn convert_wgsl() {
    let _ = env_logger::try_init();

    let inputs = [
        (
            "array-in-ctor",
            Targets::SPIRV | Targets::METAL | Targets::GLSL | Targets::HLSL | Targets::WGSL,
        ),
        (
            "array-in-function-return-type",
            Targets::SPIRV | Targets::METAL | Targets::GLSL | Targets::HLSL | Targets::WGSL,
        ),
        (
            "empty",
            Targets::SPIRV | Targets::METAL | Targets::GLSL | Targets::HLSL | Targets::WGSL,
        ),
        (
            "quad",
            Targets::SPIRV
                | Targets::METAL
                | Targets::GLSL
                | Targets::DOT
                | Targets::HLSL
                | Targets::WGSL,
        ),
        (
            "bits",
            Targets::SPIRV | Targets::METAL | Targets::GLSL | Targets::HLSL | Targets::WGSL,
        ),
        (
            "bitcast",
            Targets::SPIRV | Targets::METAL | Targets::GLSL | Targets::HLSL | Targets::WGSL,
        ),
        (
            "boids",
            Targets::SPIRV | Targets::METAL | Targets::GLSL | Targets::HLSL | Targets::WGSL,
        ),
        (
            "skybox",
            Targets::SPIRV | Targets::METAL | Targets::GLSL | Targets::HLSL | Targets::WGSL,
        ),
        (
            "collatz",
            Targets::SPIRV
                | Targets::METAL
                | Targets::IR
                | Targets::ANALYSIS
                | Targets::HLSL
                | Targets::WGSL,
        ),
        (
            "shadow",
            Targets::SPIRV | Targets::METAL | Targets::GLSL | Targets::HLSL | Targets::WGSL,
        ),
        (
            "image",
            Targets::SPIRV | Targets::METAL | Targets::HLSL | Targets::WGSL | Targets::GLSL,
        ),
        ("extra", Targets::SPIRV | Targets::METAL | Targets::WGSL),
        ("push-constants", Targets::GLSL | Targets::HLSL),
        (
            "operators",
            Targets::SPIRV | Targets::METAL | Targets::GLSL | Targets::HLSL | Targets::WGSL,
        ),
        (
            "functions",
            Targets::SPIRV | Targets::METAL | Targets::GLSL | Targets::HLSL | Targets::WGSL,
        ),
        (
            "fragment-output",
            Targets::SPIRV | Targets::METAL | Targets::GLSL | Targets::HLSL | Targets::WGSL,
        ),
        (
            "dualsource",
            Targets::SPIRV | Targets::METAL | Targets::GLSL | Targets::HLSL | Targets::WGSL,
        ),
        ("functions-webgl", Targets::GLSL),
        (
            "interpolate",
            Targets::SPIRV | Targets::METAL | Targets::HLSL | Targets::WGSL,
        ),
        (
            "interpolate_compat",
            Targets::SPIRV | Targets::METAL | Targets::GLSL | Targets::HLSL | Targets::WGSL,
        ),
        (
            "access",
            Targets::SPIRV
                | Targets::METAL
                | Targets::GLSL
                | Targets::HLSL
                | Targets::WGSL
                | Targets::IR
                | Targets::ANALYSIS,
        ),
        (
            "atomicOps",
            Targets::SPIRV | Targets::METAL | Targets::GLSL | Targets::HLSL | Targets::WGSL,
        ),
        (
            "atomicCompareExchange",
            Targets::SPIRV | Targets::METAL | Targets::WGSL,
        ),
        (
            "padding",
            Targets::SPIRV | Targets::METAL | Targets::GLSL | Targets::HLSL | Targets::WGSL,
        ),
        (
            "atomicOps-int64",
            Targets::SPIRV | Targets::HLSL | Targets::WGSL,
        ),
        (
            "atomicOps-int64-min-max",
            Targets::SPIRV | Targets::METAL | Targets::HLSL | Targets::WGSL,
        ),
        (
            "atomicTexture",
            Targets::SPIRV | Targets::METAL | Targets::GLSL | Targets::HLSL | Targets::WGSL,
        ),
        (
            "atomicOps-float32",
            Targets::SPIRV | Targets::METAL | Targets::WGSL,
        ),
        (
            "atomicTexture-int64",
            Targets::SPIRV | Targets::METAL | Targets::HLSL | Targets::WGSL,
        ),
        (
            "atomicCompareExchange-int64",
            Targets::SPIRV | Targets::WGSL,
        ),
        ("pointers", Targets::SPIRV | Targets::WGSL),
        (
            "control-flow",
            Targets::SPIRV | Targets::METAL | Targets::GLSL | Targets::HLSL | Targets::WGSL,
        ),
        (
            "standard",
            Targets::SPIRV | Targets::METAL | Targets::GLSL | Targets::HLSL | Targets::WGSL,
        ),
        //TODO: GLSL https://github.com/gfx-rs/naga/issues/874
        (
            "interface",
            Targets::SPIRV | Targets::METAL | Targets::HLSL | Targets::WGSL,
        ),
        (
            "globals",
            Targets::SPIRV | Targets::METAL | Targets::GLSL | Targets::HLSL | Targets::WGSL,
        ),
        ("bounds-check-zero", Targets::SPIRV | Targets::METAL),
        ("bounds-check-zero-atomic", Targets::METAL),
        ("bounds-check-restrict", Targets::SPIRV | Targets::METAL),
        (
            "bounds-check-image-restrict",
            Targets::SPIRV | Targets::METAL | Targets::GLSL,
        ),
        (
            "bounds-check-image-rzsw",
            Targets::SPIRV | Targets::METAL | Targets::GLSL,
        ),
        ("policy-mix", Targets::SPIRV | Targets::METAL),
        ("bounds-check-dynamic-buffer", Targets::HLSL),
        (
            "texture-arg",
            Targets::SPIRV | Targets::METAL | Targets::GLSL | Targets::HLSL | Targets::WGSL,
        ),
        ("cubeArrayShadow", Targets::GLSL),
        ("sample-cube-array-depth-lod", Targets::GLSL),
        (
            "use-gl-ext-over-grad-workaround-if-instructed",
            Targets::GLSL,
        ),
        ("local-const", Targets::IR | Targets::WGSL),
        (
            "math-functions",
            Targets::SPIRV | Targets::METAL | Targets::GLSL | Targets::HLSL | Targets::WGSL,
        ),
        (
            "binding-arrays",
            Targets::WGSL | Targets::HLSL | Targets::METAL | Targets::SPIRV,
        ),
        (
            "binding-buffer-arrays",
            Targets::WGSL | Targets::SPIRV, //TODO: more backends, eventually merge into "binding-arrays"
        ),
        ("resource-binding-map", Targets::METAL),
        ("multiview", Targets::SPIRV | Targets::GLSL | Targets::WGSL),
        ("multiview_webgl", Targets::GLSL),
        (
            "break-if",
            Targets::WGSL | Targets::GLSL | Targets::SPIRV | Targets::HLSL | Targets::METAL,
        ),
        ("lexical-scopes", Targets::WGSL),
        ("type-alias", Targets::WGSL),
        ("module-scope", Targets::WGSL),
        (
            "workgroup-var-init",
            Targets::WGSL | Targets::GLSL | Targets::SPIRV | Targets::HLSL | Targets::METAL,
        ),
        (
            "workgroup-uniform-load",
            Targets::WGSL | Targets::GLSL | Targets::SPIRV | Targets::HLSL | Targets::METAL,
        ),
        ("runtime-array-in-unused-struct", Targets::SPIRV),
        ("sprite", Targets::SPIRV),
        ("force_point_size_vertex_shader_webgl", Targets::GLSL),
        ("invariant", Targets::GLSL),
        ("ray-query", Targets::SPIRV | Targets::METAL | Targets::HLSL),
        ("hlsl-keyword", Targets::HLSL),
        (
            "constructors",
            Targets::SPIRV | Targets::METAL | Targets::GLSL | Targets::HLSL | Targets::WGSL,
        ),
        ("msl-varyings", Targets::METAL),
        (
            "const-exprs",
            Targets::SPIRV | Targets::METAL | Targets::GLSL | Targets::HLSL | Targets::WGSL,
        ),
        ("const_assert", Targets::WGSL | Targets::IR),
        ("separate-entry-points", Targets::SPIRV | Targets::GLSL),
        (
            "struct-layout",
            Targets::WGSL | Targets::GLSL | Targets::SPIRV | Targets::HLSL | Targets::METAL,
        ),
        (
            "f64",
            Targets::SPIRV | Targets::GLSL | Targets::HLSL | Targets::WGSL,
        ),
        (
            "abstract-types-const",
            Targets::SPIRV | Targets::METAL | Targets::GLSL | Targets::WGSL,
        ),
        (
            "abstract-types-function-calls",
            Targets::SPIRV | Targets::METAL | Targets::GLSL | Targets::WGSL,
        ),
        (
            "abstract-types-var",
            Targets::SPIRV | Targets::METAL | Targets::GLSL | Targets::WGSL,
        ),
        (
            "abstract-types-operators",
            Targets::SPIRV | Targets::METAL | Targets::GLSL | Targets::WGSL,
        ),
        (
            "abstract-types-return",
            Targets::SPIRV | Targets::METAL | Targets::GLSL | Targets::HLSL | Targets::WGSL,
        ),
        (
            "int64",
            Targets::SPIRV | Targets::HLSL | Targets::WGSL | Targets::METAL,
        ),
        (
            "subgroup-operations",
            Targets::SPIRV | Targets::METAL | Targets::GLSL | Targets::HLSL | Targets::WGSL,
        ),
        (
            "overrides",
            Targets::IR
                | Targets::ANALYSIS
                | Targets::SPIRV
                | Targets::METAL
                | Targets::HLSL
                | Targets::GLSL,
        ),
        (
            "overrides-atomicCompareExchangeWeak",
            Targets::IR | Targets::SPIRV | Targets::METAL,
        ),
        (
            "overrides-ray-query",
            Targets::IR | Targets::SPIRV | Targets::METAL,
        ),
        ("vertex-pulling-transform", Targets::METAL),
        (
            "cross",
            Targets::SPIRV | Targets::METAL | Targets::GLSL | Targets::HLSL | Targets::WGSL,
        ),
        (
            "phony_assignment",
            Targets::SPIRV | Targets::METAL | Targets::GLSL | Targets::HLSL | Targets::WGSL,
        ),
        ("6220-break-from-loop", Targets::SPIRV),
        ("index-by-value", Targets::SPIRV | Targets::IR),
        (
            "6438-conflicting-idents",
            Targets::SPIRV | Targets::METAL | Targets::GLSL | Targets::HLSL | Targets::WGSL,
        ),
        ("diagnostic-filter", Targets::IR),
        (
            "6772-unpack-expr-accesses",
            Targets::SPIRV | Targets::METAL | Targets::GLSL | Targets::HLSL | Targets::WGSL,
        ),
        ("must-use", Targets::IR),
        (
            "storage-textures",
            Targets::IR | Targets::ANALYSIS | Targets::SPIRV | Targets::METAL | Targets::HLSL,
        ),
    ];

    for &(name, targets) in inputs.iter() {
        // WGSL shaders lives in root dir as a privileged.
        let input = Input::new(None, name, "wgsl");
        let source = input.read_source();
        match naga::front::wgsl::parse_str(&source) {
            Ok(mut module) => check_targets(&input, &mut module, targets, None, None),
            Err(e) => panic!(
                "{}",
                e.emit_to_string_with_path(&source, input.input_path())
            ),
        }
    }

    {
        let inputs = [
            ("debug-symbol-simple", Targets::SPIRV),
            ("debug-symbol-terrain", Targets::SPIRV),
            ("debug-symbol-large-source", Targets::SPIRV),
        ];
        for &(name, targets) in inputs.iter() {
            // WGSL shaders lives in root dir as a privileged.
            let input = Input::new(None, name, "wgsl");
            let source = input.read_source();

            // crlf will make the large split output different on different platform
            let source = source.replace('\r', "");
            match naga::front::wgsl::parse_str(&source) {
                Ok(mut module) => check_targets(&input, &mut module, targets, Some(&source), None),
                Err(e) => panic!(
                    "{}",
                    e.emit_to_string_with_path(&source, input.input_path())
                ),
            }
        }
    }
}

#[cfg(all(feature = "wgsl-in", hlsl_out))]
#[test]
fn unconsumed_vertex_outputs_hlsl_out() {
    let load_and_parse = |name| {
        // WGSL shaders lives in root dir as a privileged.
        let input = Input::new(None, name, "wgsl");
        let source = input.read_source();
        let module = match naga::front::wgsl::parse_str(&source) {
            Ok(module) => module,
            Err(e) => panic!(
                "{}",
                e.emit_to_string_with_path(&source, input.input_path())
            ),
        };
        (input, module)
    };

    // Uses separate wgsl files to make sure the tested code doesn't accidentally rely on
    // the fragment entry point being from the same parsed content (e.g. accidentally using the
    // wrong `Module` when looking up info). We also don't just create a module from the same file
    // twice since everything would probably be stored behind the same keys.
    let (input, mut module) = load_and_parse("unconsumed_vertex_outputs_vert");
    let (frag_input, mut frag_module) = load_and_parse("unconsumed_vertex_outputs_frag");
    let frag_ep = naga::back::hlsl::FragmentEntryPoint::new(&frag_module, "fs_main")
        .expect("fs_main not found");

    check_targets(&input, &mut module, Targets::HLSL, None, Some(frag_ep));
    check_targets(&frag_input, &mut frag_module, Targets::HLSL, None, None);
}

#[cfg(feature = "spv-in")]
fn convert_spv(name: &str, adjust_coordinate_space: bool, targets: Targets) {
    let _ = env_logger::try_init();

    let input = Input::new(Some("spv"), name, "spv");
    let mut module = naga::front::spv::parse_u8_slice(
        &input.read_bytes(),
        &naga::front::spv::Options {
            adjust_coordinate_space,
            strict_capabilities: false,
            block_ctx_dump_prefix: None,
        },
    )
    .unwrap();
    check_targets(&input, &mut module, targets, None, None);
}

#[cfg(feature = "spv-in")]
#[test]
fn convert_spv_all() {
    convert_spv(
        "quad-vert",
        false,
        Targets::METAL | Targets::GLSL | Targets::HLSL | Targets::WGSL,
    );
    convert_spv("shadow", true, Targets::IR | Targets::ANALYSIS);
    convert_spv(
        "inv-hyperbolic-trig-functions",
        true,
        Targets::HLSL | Targets::WGSL,
    );
    convert_spv(
        "empty-global-name",
        true,
        Targets::HLSL | Targets::WGSL | Targets::METAL,
    );
    convert_spv("degrees", false, Targets::empty());
    convert_spv("binding-arrays.dynamic", true, Targets::WGSL);
    convert_spv("binding-arrays.static", true, Targets::WGSL);
    convert_spv(
        "do-while",
        true,
        Targets::METAL | Targets::GLSL | Targets::HLSL | Targets::WGSL,
    );
    convert_spv(
        "unnamed-gl-per-vertex",
        true,
        Targets::METAL | Targets::GLSL | Targets::HLSL | Targets::WGSL,
    );
    convert_spv("builtin-accessed-outside-entrypoint", true, Targets::WGSL);
    convert_spv("spec-constants", true, Targets::IR);
    convert_spv("spec-constants-issue-5598", true, Targets::GLSL);
    convert_spv(
        "subgroup-operations-s",
        false,
        Targets::METAL | Targets::GLSL | Targets::HLSL | Targets::WGSL,
    );
    convert_spv("atomic_i_increment", false, Targets::WGSL);
    convert_spv("atomic_load_and_store", false, Targets::WGSL);
    convert_spv("atomic_exchange", false, Targets::WGSL);
    convert_spv("atomic_compare_exchange", false, Targets::WGSL);
    convert_spv("atomic_i_decrement", false, Targets::WGSL);
    convert_spv("atomic_i_add_sub", false, Targets::WGSL);
    convert_spv("atomic_global_struct_field_vertex", false, Targets::WGSL);
    convert_spv(
        "fetch_depth",
        false,
        Targets::IR | Targets::SPIRV | Targets::METAL | Targets::HLSL | Targets::WGSL,
    );
}

#[cfg(feature = "glsl-in")]
#[test]
fn convert_glsl_variations_check() {
    let input = Input::new(None, "variations", "glsl");
    let source = input.read_source();
    let mut parser = naga::front::glsl::Frontend::default();
    let mut module = parser
        .parse(
            &naga::front::glsl::Options {
                stage: naga::ShaderStage::Fragment,
                defines: Default::default(),
            },
            &source,
        )
        .unwrap();
    check_targets(&input, &mut module, Targets::GLSL, None, None);
}

#[cfg(feature = "glsl-in")]
#[allow(unused_variables)]
#[test]
fn convert_glsl_folder() {
    let _ = env_logger::try_init();

    for input in Input::files_in_dir("glsl") {
        let input = Input {
            keep_input_extension: true,
            ..input
        };
        let file_name = &input.file_name;
        if file_name.ends_with(".ron") {
            // No needed to validate ron files
            continue;
        }

        let mut parser = naga::front::glsl::Frontend::default();
        let module = parser
            .parse(
                &naga::front::glsl::Options {
                    stage: match file_name.extension().and_then(|s| s.to_str()).unwrap() {
                        "vert" => naga::ShaderStage::Vertex,
                        "frag" => naga::ShaderStage::Fragment,
                        "comp" => naga::ShaderStage::Compute,
                        ext => panic!("Unknown extension for glsl file {ext}"),
                    },
                    defines: Default::default(),
                },
                &input.read_source(),
            )
            .unwrap();

        let info = naga::valid::Validator::new(
            naga::valid::ValidationFlags::all(),
            naga::valid::Capabilities::all(),
        )
        .validate(&module)
        .unwrap();

        #[cfg(wgsl_out)]
        {
            write_output_wgsl(&input, &module, &info, &WgslOutParameters::default());
        }
    }
}
