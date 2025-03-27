// Keep in sync with bits.wgsl

@fragment
fn main() {
    var i = 0;
    var i2 = vec2<i32>(0);
    var i3 = vec3<i32>(0);
    var i4 = vec4<i32>(0);
    var u = 0u;
    var u2 = vec2<u32>(0u);
    var u3 = vec3<u32>(0u);
    var u4 = vec4<u32>(0u);
    var f2 = vec2<f32>(0.0);
    var f4 = vec4<f32>(0.0);
    // No polyfill for these yet
    // u = pack4x8snorm(f4);
    // u = pack4x8unorm(f4);
    // u = pack2x16snorm(f2);
    // u = pack2x16unorm(f2);
    // u = pack2x16float(f2);
    u = pack4xI8(i4);
    u = pack4xU8(u4);
    f4 = unpack4x8snorm(u);
    f4 = unpack4x8unorm(u);
    f2 = unpack2x16snorm(u);
    f2 = unpack2x16unorm(u);
    // No polyfill for this yet
    // f2 = unpack2x16float(u);
    // Polyfill for this is broken in downlevel
    // i4 = unpack4xI8(u);
    // u4 = unpack4xU8(u);
    // Implementation is broken on downlevel
    // i = insertBits(i, i, 5u, 10u);
    // i2 = insertBits(i2, i2, 5u, 10u);
    // i3 = insertBits(i3, i3, 5u, 10u);
    // i4 = insertBits(i4, i4, 5u, 10u);
    // u = insertBits(u, u, 5u, 10u);
    // u2 = insertBits(u2, u2, 5u, 10u);
    // u3 = insertBits(u3, u3, 5u, 10u);
    // u4 = insertBits(u4, u4, 5u, 10u);
    // Implementation is broken on downlevel
    // i = extractBits(i, 5u, 10u);
    // i2 = extractBits(i2, 5u, 10u);
    // i3 = extractBits(i3, 5u, 10u);
    // i4 = extractBits(i4, 5u, 10u);
    // u = extractBits(u, 5u, 10u);
    // u2 = extractBits(u2, 5u, 10u);
    // u3 = extractBits(u3, 5u, 10u);
    // u4 = extractBits(u4, 5u, 10u);
    // Implementation is broken on downlevel
    // i = firstTrailingBit(i);
    // u2 = firstTrailingBit(u2);
    // i3 = firstLeadingBit(i3);
    // u3 = firstLeadingBit(u3);
    // Implementation is broken on downlevel
    // i = firstLeadingBit(i);
    // u = firstLeadingBit(u);
    // Implementation is broken on downlevel
    // i = countOneBits(i);
    // i2 = countOneBits(i2);
    // i3 = countOneBits(i3);
    // i4 = countOneBits(i4);
    // u = countOneBits(u);
    // u2 = countOneBits(u2);
    // u3 = countOneBits(u3);
    // u4 = countOneBits(u4);
    // Implementation is broken on downlevel
    // i = reverseBits(i);
    // i2 = reverseBits(i2);
    // i3 = reverseBits(i3);
    // i4 = reverseBits(i4);
    // u = reverseBits(u);
    // u2 = reverseBits(u2);
    // u3 = reverseBits(u3);
    // u4 = reverseBits(u4);
}
