var<private> xvipaiai_1: vec2<i32> = vec2<i32>(42i, 43i);
var<private> xvupaiai_1: vec2<u32> = vec2<u32>(44u, 45u);
var<private> xvfpaiai_1: vec2<f32> = vec2<f32>(46f, 47f);
var<private> xvfpafaf_1: vec2<f32> = vec2<f32>(48f, 49f);
var<private> xvfpaiaf_1: vec2<f32> = vec2<f32>(48f, 49f);
var<private> xvupuai_2: vec2<u32> = vec2<u32>(42u, 43u);
var<private> xvupaiu_2: vec2<u32> = vec2<u32>(42u, 43u);
var<private> xvuuai_2: vec2<u32> = vec2<u32>(42u, 43u);
var<private> xvuaiu_2: vec2<u32> = vec2<u32>(42u, 43u);
var<private> xvip_1: vec2<i32> = vec2<i32>(0i, 0i);
var<private> xvup_1: vec2<u32> = vec2<u32>(0u, 0u);
var<private> xvfp_1: vec2<f32> = vec2<f32>(0f, 0f);
var<private> xmfp_1: mat2x2<f32> = mat2x2<f32>(vec2<f32>(0f, 0f), vec2<f32>(0f, 0f));
var<private> xmfpaiaiaiai_1: mat2x2<f32> = mat2x2<f32>(vec2<f32>(1f, 2f), vec2<f32>(3f, 4f));
var<private> xmfpafaiaiai_1: mat2x2<f32> = mat2x2<f32>(vec2<f32>(1f, 2f), vec2<f32>(3f, 4f));
var<private> xmfpaiafaiai_1: mat2x2<f32> = mat2x2<f32>(vec2<f32>(1f, 2f), vec2<f32>(3f, 4f));
var<private> xmfpaiaiafai_1: mat2x2<f32> = mat2x2<f32>(vec2<f32>(1f, 2f), vec2<f32>(3f, 4f));
var<private> xmfpaiaiaiaf_1: mat2x2<f32> = mat2x2<f32>(vec2<f32>(1f, 2f), vec2<f32>(3f, 4f));
var<private> xvispai_1: vec2<i32> = vec2(1i);
var<private> xvfspaf_1: vec2<f32> = vec2(1f);
var<private> xvis_ai_1: vec2<i32> = vec2(1i);
var<private> xvus_ai_1: vec2<u32> = vec2(1u);
var<private> xvfs_ai_1: vec2<f32> = vec2(1f);
var<private> xvfs_af_1: vec2<f32> = vec2(1f);
var<private> xafafaf_1: array<f32, 2> = array<f32, 2>(1f, 2f);
var<private> xafaiai_1: array<f32, 2> = array<f32, 2>(1f, 2f);
var<private> xaipaiai_1: array<i32, 2> = array<i32, 2>(1i, 2i);
var<private> xaupaiai: array<u32, 2> = array<u32, 2>(1u, 2u);
var<private> xafpaiaf_1: array<f32, 2> = array<f32, 2>(1f, 2f);
var<private> xafpafai_1: array<f32, 2> = array<f32, 2>(1f, 2f);
var<private> xafpafaf_1: array<f32, 2> = array<f32, 2>(1f, 2f);
var<private> xavipai_1: array<vec3<i32>, 1> = array<vec3<i32>, 1>(vec3(1i));
var<private> xavfpai_1: array<vec3<f32>, 1> = array<vec3<f32>, 1>(vec3(1f));
var<private> xavfpaf_1: array<vec3<f32>, 1> = array<vec3<f32>, 1>(vec3(1f));
var<private> xvisai_1: vec2<i32> = vec2(1i);
var<private> xvusai_1: vec2<u32> = vec2(1u);
var<private> xvfsai_1: vec2<f32> = vec2(1f);
var<private> xvfsaf_1: vec2<f32> = vec2(1f);
var<private> ivispai: vec2<i32> = vec2(1i);
var<private> ivfspaf: vec2<f32> = vec2(1f);
var<private> ivis_ai: vec2<i32> = vec2(1i);
var<private> ivus_ai: vec2<u32> = vec2(1u);
var<private> ivfs_ai: vec2<f32> = vec2(1f);
var<private> ivfs_af: vec2<f32> = vec2(1f);
var<private> iafafaf: array<f32, 2> = array<f32, 2>(1f, 2f);
var<private> iafaiai: array<f32, 2> = array<f32, 2>(1f, 2f);
var<private> iaipaiai_1: array<i32, 2> = array<i32, 2>(1i, 2i);
var<private> iafpafaf_1: array<f32, 2> = array<f32, 2>(1f, 2f);
var<private> iafpaiaf_1: array<f32, 2> = array<f32, 2>(1f, 2f);
var<private> iafpafai_1: array<f32, 2> = array<f32, 2>(1f, 2f);
var<private> iavipai: array<vec3<i32>, 1> = array<vec3<i32>, 1>(vec3(1i));
var<private> iavfpai: array<vec3<i32>, 1> = array<vec3<i32>, 1>(vec3(1i));
var<private> iavfpaf: array<vec3<f32>, 1> = array<vec3<f32>, 1>(vec3(1f));

fn all_constant_arguments() {
    var xvipaiai: vec2<i32> = vec2<i32>(42i, 43i);
    var xvupaiai: vec2<u32> = vec2<u32>(44u, 45u);
    var xvfpaiai: vec2<f32> = vec2<f32>(46f, 47f);
    var xvfpafaf: vec2<f32> = vec2<f32>(48f, 49f);
    var xvfpaiaf: vec2<f32> = vec2<f32>(48f, 49f);
    var xvupuai: vec2<u32> = vec2<u32>(42u, 43u);
    var xvupaiu: vec2<u32> = vec2<u32>(42u, 43u);
    var xvuuai: vec2<u32> = vec2<u32>(42u, 43u);
    var xvuaiu: vec2<u32> = vec2<u32>(42u, 43u);
    var xvip: vec2<i32> = vec2<i32>(0i, 0i);
    var xvup: vec2<u32> = vec2<u32>(0u, 0u);
    var xvfp: vec2<f32> = vec2<f32>(0f, 0f);
    var xmfp: mat2x2<f32> = mat2x2<f32>(vec2<f32>(0f, 0f), vec2<f32>(0f, 0f));
    var xmfpaiaiaiai: mat2x2<f32> = mat2x2<f32>(vec2<f32>(1f, 2f), vec2<f32>(3f, 4f));
    var xmfpafaiaiai: mat2x2<f32> = mat2x2<f32>(vec2<f32>(1f, 2f), vec2<f32>(3f, 4f));
    var xmfpaiafaiai: mat2x2<f32> = mat2x2<f32>(vec2<f32>(1f, 2f), vec2<f32>(3f, 4f));
    var xmfpaiaiafai: mat2x2<f32> = mat2x2<f32>(vec2<f32>(1f, 2f), vec2<f32>(3f, 4f));
    var xmfpaiaiaiaf: mat2x2<f32> = mat2x2<f32>(vec2<f32>(1f, 2f), vec2<f32>(3f, 4f));
    var xmfp_faiaiai: mat2x2<f32> = mat2x2<f32>(vec2<f32>(1f, 2f), vec2<f32>(3f, 4f));
    var xmfpai_faiai: mat2x2<f32> = mat2x2<f32>(vec2<f32>(1f, 2f), vec2<f32>(3f, 4f));
    var xmfpaiai_fai: mat2x2<f32> = mat2x2<f32>(vec2<f32>(1f, 2f), vec2<f32>(3f, 4f));
    var xmfpaiaiai_f: mat2x2<f32> = mat2x2<f32>(vec2<f32>(1f, 2f), vec2<f32>(3f, 4f));
    var xvispai: vec2<i32> = vec2(1i);
    var xvfspaf: vec2<f32> = vec2(1f);
    var xvis_ai: vec2<i32> = vec2(1i);
    var xvus_ai: vec2<u32> = vec2(1u);
    var xvfs_ai: vec2<f32> = vec2(1f);
    var xvfs_af: vec2<f32> = vec2(1f);
    var xafafaf: array<f32, 2> = array<f32, 2>(1f, 2f);
    var xaf_faf: array<f32, 2> = array<f32, 2>(1f, 2f);
    var xafaf_f: array<f32, 2> = array<f32, 2>(1f, 2f);
    var xafaiai: array<f32, 2> = array<f32, 2>(1f, 2f);
    var xai_iai: array<i32, 2> = array<i32, 2>(1i, 2i);
    var xaiai_i: array<i32, 2> = array<i32, 2>(1i, 2i);
    var xaipaiai: array<i32, 2> = array<i32, 2>(1i, 2i);
    var xafpaiai: array<f32, 2> = array<f32, 2>(1f, 2f);
    var xafpaiaf: array<f32, 2> = array<f32, 2>(1f, 2f);
    var xafpafai: array<f32, 2> = array<f32, 2>(1f, 2f);
    var xafpafaf: array<f32, 2> = array<f32, 2>(1f, 2f);
    var xavipai: array<vec3<i32>, 1> = array<vec3<i32>, 1>(vec3(1i));
    var xavfpai: array<vec3<f32>, 1> = array<vec3<f32>, 1>(vec3(1f));
    var xavfpaf: array<vec3<f32>, 1> = array<vec3<f32>, 1>(vec3(1f));
    var xvisai: vec2<i32> = vec2(1i);
    var xvusai: vec2<u32> = vec2(1u);
    var xvfsai: vec2<f32> = vec2(1f);
    var xvfsaf: vec2<f32> = vec2(1f);
    var iaipaiai: array<i32, 2> = array<i32, 2>(1i, 2i);
    var iafpaiaf: array<f32, 2> = array<f32, 2>(1f, 2f);
    var iafpafai: array<f32, 2> = array<f32, 2>(1f, 2f);
    var iafpafaf: array<f32, 2> = array<f32, 2>(1f, 2f);

    xvipaiai = vec2<i32>(42i, 43i);
    xvupaiai = vec2<u32>(44u, 45u);
    xvfpaiai = vec2<f32>(46f, 47f);
    xvfpafaf = vec2<f32>(48f, 49f);
    xvfpaiaf = vec2<f32>(48f, 49f);
    xvupuai = vec2<u32>(42u, 43u);
    xvupaiu = vec2<u32>(42u, 43u);
    xvuuai = vec2<u32>(42u, 43u);
    xvuaiu = vec2<u32>(42u, 43u);
    xvip = vec2<i32>(0i, 0i);
    xvup = vec2<u32>(0u, 0u);
    xvfp = vec2<f32>(0f, 0f);
    xmfp = mat2x2<f32>(vec2<f32>(0f, 0f), vec2<f32>(0f, 0f));
    xmfpaiaiaiai = mat2x2<f32>(vec2<f32>(1f, 2f), vec2<f32>(3f, 4f));
    xmfpafaiaiai = mat2x2<f32>(vec2<f32>(1f, 2f), vec2<f32>(3f, 4f));
    xmfpaiafaiai = mat2x2<f32>(vec2<f32>(1f, 2f), vec2<f32>(3f, 4f));
    xmfpaiaiafai = mat2x2<f32>(vec2<f32>(1f, 2f), vec2<f32>(3f, 4f));
    xmfpaiaiaiaf = mat2x2<f32>(vec2<f32>(1f, 2f), vec2<f32>(3f, 4f));
    xmfp_faiaiai = mat2x2<f32>(vec2<f32>(1f, 2f), vec2<f32>(3f, 4f));
    xmfpai_faiai = mat2x2<f32>(vec2<f32>(1f, 2f), vec2<f32>(3f, 4f));
    xmfpaiai_fai = mat2x2<f32>(vec2<f32>(1f, 2f), vec2<f32>(3f, 4f));
    xmfpaiaiai_f = mat2x2<f32>(vec2<f32>(1f, 2f), vec2<f32>(3f, 4f));
    xvispai = vec2(1i);
    xvfspaf = vec2(1f);
    xvis_ai = vec2(1i);
    xvus_ai = vec2(1u);
    xvfs_ai = vec2(1f);
    xvfs_af = vec2(1f);
    xafafaf = array<f32, 2>(1f, 2f);
    xaf_faf = array<f32, 2>(1f, 2f);
    xafaf_f = array<f32, 2>(1f, 2f);
    xafaiai = array<f32, 2>(1f, 2f);
    xai_iai = array<i32, 2>(1i, 2i);
    xaiai_i = array<i32, 2>(1i, 2i);
    xaipaiai = array<i32, 2>(1i, 2i);
    xafpaiai = array<f32, 2>(1f, 2f);
    xafpaiaf = array<f32, 2>(1f, 2f);
    xafpafai = array<f32, 2>(1f, 2f);
    xafpafaf = array<f32, 2>(1f, 2f);
    xavipai = array<vec3<i32>, 1>(vec3(1i));
    xavfpai = array<vec3<f32>, 1>(vec3(1f));
    xavfpaf = array<vec3<f32>, 1>(vec3(1f));
    xvisai = vec2(1i);
    xvusai = vec2(1u);
    xvfsai = vec2(1f);
    xvfsaf = vec2(1f);
    iaipaiai = array<i32, 2>(1i, 2i);
    iafpaiaf = array<f32, 2>(1f, 2f);
    iafpafai = array<f32, 2>(1f, 2f);
    iafpafaf = array<f32, 2>(1f, 2f);
    return;
}

fn mixed_constant_and_runtime_arguments() {
    var u: u32;
    var i: i32;
    var f: f32;
    var xvupuai_1: vec2<u32>;
    var xvupaiu_1: vec2<u32>;
    var xvfpfai: vec2<f32>;
    var xvfpfaf: vec2<f32>;
    var xvuuai_1: vec2<u32>;
    var xvuaiu_1: vec2<u32>;
    var xmfp_faiaiai_1: mat2x2<f32>;
    var xmfpai_faiai_1: mat2x2<f32>;
    var xmfpaiai_fai_1: mat2x2<f32>;
    var xmfpaiaiai_f_1: mat2x2<f32>;
    var xaf_faf_1: array<f32, 2>;
    var xafaf_f_1: array<f32, 2>;
    var xaf_fai: array<f32, 2>;
    var xafai_f: array<f32, 2>;
    var xai_iai_1: array<i32, 2>;
    var xaiai_i_1: array<i32, 2>;
    var xafp_faf: array<f32, 2>;
    var xafpaf_f: array<f32, 2>;
    var xafp_fai: array<f32, 2>;
    var xafpai_f: array<f32, 2>;
    var xaip_iai: array<i32, 2>;
    var xaipai_i: array<i32, 2>;
    var xvisi: vec2<i32>;
    var xvusu: vec2<u32>;
    var xvfsf: vec2<f32>;

    let _e3 = u;
    xvupuai_1 = vec2<u32>(_e3, 43u);
    let _e7 = u;
    xvupaiu_1 = vec2<u32>(42u, _e7);
    let _e11 = f;
    xvfpfai = vec2<f32>(_e11, 47f);
    let _e15 = f;
    xvfpfaf = vec2<f32>(_e15, 49f);
    let _e19 = u;
    xvuuai_1 = vec2<u32>(_e19, 43u);
    let _e23 = u;
    xvuaiu_1 = vec2<u32>(42u, _e23);
    let _e27 = f;
    xmfp_faiaiai_1 = mat2x2<f32>(vec2<f32>(_e27, 2f), vec2<f32>(3f, 4f));
    let _e35 = f;
    xmfpai_faiai_1 = mat2x2<f32>(vec2<f32>(1f, _e35), vec2<f32>(3f, 4f));
    let _e43 = f;
    xmfpaiai_fai_1 = mat2x2<f32>(vec2<f32>(1f, 2f), vec2<f32>(_e43, 4f));
    let _e51 = f;
    xmfpaiaiai_f_1 = mat2x2<f32>(vec2<f32>(1f, 2f), vec2<f32>(3f, _e51));
    let _e59 = f;
    xaf_faf_1 = array<f32, 2>(_e59, 2f);
    let _e63 = f;
    xafaf_f_1 = array<f32, 2>(1f, _e63);
    let _e67 = f;
    xaf_fai = array<f32, 2>(_e67, 2f);
    let _e71 = f;
    xafai_f = array<f32, 2>(1f, _e71);
    let _e75 = i;
    xai_iai_1 = array<i32, 2>(_e75, 2i);
    let _e79 = i;
    xaiai_i_1 = array<i32, 2>(1i, _e79);
    let _e83 = f;
    xafp_faf = array<f32, 2>(_e83, 2f);
    let _e87 = f;
    xafpaf_f = array<f32, 2>(1f, _e87);
    let _e91 = f;
    xafp_fai = array<f32, 2>(_e91, 2f);
    let _e95 = f;
    xafpai_f = array<f32, 2>(1f, _e95);
    let _e99 = i;
    xaip_iai = array<i32, 2>(_e99, 2i);
    let _e103 = i;
    xaipai_i = array<i32, 2>(1i, _e103);
    let _e107 = i;
    xvisi = vec2(_e107);
    let _e110 = u;
    xvusu = vec2(_e110);
    let _e113 = f;
    xvfsf = vec2(_e113);
    let _e116 = u;
    xvupuai_1 = vec2<u32>(_e116, 43u);
    let _e119 = u;
    xvupaiu_1 = vec2<u32>(42u, _e119);
    let _e122 = u;
    xvuuai_1 = vec2<u32>(_e122, 43u);
    let _e125 = u;
    xvuaiu_1 = vec2<u32>(42u, _e125);
    let _e128 = f;
    xmfp_faiaiai_1 = mat2x2<f32>(vec2<f32>(_e128, 2f), vec2<f32>(3f, 4f));
    let _e135 = f;
    xmfpai_faiai_1 = mat2x2<f32>(vec2<f32>(1f, _e135), vec2<f32>(3f, 4f));
    let _e142 = f;
    xmfpaiai_fai_1 = mat2x2<f32>(vec2<f32>(1f, 2f), vec2<f32>(_e142, 4f));
    let _e149 = f;
    xmfpaiaiai_f_1 = mat2x2<f32>(vec2<f32>(1f, 2f), vec2<f32>(3f, _e149));
    let _e156 = f;
    xaf_faf_1 = array<f32, 2>(_e156, 2f);
    let _e159 = f;
    xafaf_f_1 = array<f32, 2>(1f, _e159);
    let _e162 = f;
    xaf_fai = array<f32, 2>(_e162, 2f);
    let _e165 = f;
    xafai_f = array<f32, 2>(1f, _e165);
    let _e168 = i;
    xai_iai_1 = array<i32, 2>(_e168, 2i);
    let _e171 = i;
    xaiai_i_1 = array<i32, 2>(1i, _e171);
    let _e174 = f;
    xafp_faf = array<f32, 2>(_e174, 2f);
    let _e177 = f;
    xafpaf_f = array<f32, 2>(1f, _e177);
    let _e180 = f;
    xafp_fai = array<f32, 2>(_e180, 2f);
    let _e183 = f;
    xafpai_f = array<f32, 2>(1f, _e183);
    let _e186 = i;
    xaip_iai = array<i32, 2>(_e186, 2i);
    let _e189 = i;
    xaipai_i = array<i32, 2>(1i, _e189);
    let _e192 = i;
    xvisi = vec2(_e192);
    let _e194 = u;
    xvusu = vec2(_e194);
    let _e196 = f;
    xvfsf = vec2(_e196);
    return;
}

