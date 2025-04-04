struct FragmentOutput {
    @location(0) o_color: vec4<f32>,
}

var<private> o_color: vec4<f32>;

fn TevPerCompGT(a: f32, b: f32) -> f32 {
    var a_1: f32;
    var b_1: f32;

    a_1 = a;
    b_1 = b;
    let _e4 = a_1;
    let _e5 = b_1;
    return select(0f, 1f, (_e4 > _e5));
}

fn TevPerCompGT_1(a_2: vec3<f32>, b_2: vec3<f32>) -> vec3<f32> {
    var a_3: vec3<f32>;
    var b_3: vec3<f32>;

    a_3 = a_2;
    b_3 = b_2;
    let _e4 = a_3;
    let _e5 = b_3;
    return select(vec3(0f), vec3(1f), (_e4 > _e5));
}

fn main_1() {
    let _e5 = TevPerCompGT_1(vec3(3f), vec3(5f));
    o_color.x = _e5.x;
    o_color.y = _e5.y;
    o_color.z = _e5.z;
    let _e15 = TevPerCompGT(3f, 5f);
    o_color.w = _e15;
    return;
}

@fragment 
fn main() -> FragmentOutput {
    main_1();
    let _e1 = o_color;
    return FragmentOutput(_e1);
}
