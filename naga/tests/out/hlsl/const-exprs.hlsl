static const uint TWO = 2u;
static const int THREE = int(3);
static const int FOUR = int(4);
static const int FOUR_ALIAS = int(4);
static const int TEST_CONSTANT_ADDITION = int(8);
static const int TEST_CONSTANT_ALIAS_ADDITION = int(8);
static const float PI = 3.141;
static const float phi_sun = 6.282;
static const float4 DIV = float4(0.44444445, 0.0, 0.0, 0.0);
static const int TEXTURE_KIND_REGULAR = int(0);
static const int TEXTURE_KIND_WARP = int(1);
static const int TEXTURE_KIND_SKY = int(2);
static const float2 add_vec = float2(4.0, 5.0);
static const bool2 compare_vec = bool2(true, false);

void swizzle_of_compose()
{
    int4 out_ = int4(int(4), int(3), int(2), int(1));

    return;
}

void index_of_compose()
{
    int out_1 = int(2);

    return;
}

void compose_three_deep()
{
    int out_2 = int(6);

    return;
}

void non_constant_initializers()
{
    int w = int(30);
    int x = (int)0;
    int y = (int)0;
    int z = int(70);
    int4 out_3 = (int4)0;

    int _e2 = w;
    x = _e2;
    int _e4 = x;
    y = _e4;
    int _e8 = w;
    int _e9 = x;
    int _e10 = y;
    int _e11 = z;
    out_3 = int4(_e8, _e9, _e10, _e11);
    return;
}

void splat_of_constant()
{
    int4 out_4 = int4(int(-4), int(-4), int(-4), int(-4));

    return;
}

void compose_of_constant()
{
    int4 out_5 = int4(int(-4), int(-4), int(-4), int(-4));

    return;
}

void compose_of_splat()
{
    float4 x_1 = float4(2.0, 1.0, 1.0, 1.0);

    return;
}

uint map_texture_kind(int texture_kind)
{
    switch(texture_kind) {
        case 0: {
            return 10u;
        }
        case 1: {
            return 20u;
        }
        case 2: {
            return 30u;
        }
        default: {
            return 0u;
        }
    }
}

void compose_vector_zero_val_binop()
{
    int3 a = int3(int(1), int(1), int(1));
    int3 b = int3(int(0), int(1), int(2));
    int3 c = int3(int(1), int(0), int(2));

    return;
}

[numthreads(2, 3, 1)]
void main()
{
    swizzle_of_compose();
    index_of_compose();
    compose_three_deep();
    non_constant_initializers();
    splat_of_constant();
    compose_of_constant();
    compose_of_splat();
    return;
}
