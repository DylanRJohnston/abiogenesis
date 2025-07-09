#define_import_path utils::math

fn lcg(seed: u32) -> u32 {
    var result = seed;

    result ^= 2747636419u;
    result *= 2654435769u;
    result ^= result >> 16;
    result *= 2654435769u;
    result ^= result >> 16;
    result *= 2654435769u;

    return result;
}

fn random_float(seed: u32) -> f32 {
    let rand = lcg(seed);
    return f32(rand) / f32(0xffffffffu);
}

fn clamp_length(value: vec2<f32>, min: f32, max: f32) -> vec2<f32> {
    let len = length(value);

    if len == 0.0 {
        return vec2f(0.0);
    } else {
        return value * clamp(len, min, max) / len;
    }
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    return a + (b - a) * t;
}

fn inverse_lerp(a: f32, b: f32, value: f32) -> f32 {
    return (value - a) / (b - a);
}

fn remap(value: f32, a: f32, b: f32, c: f32, d: f32) -> f32 {
    return lerp(c, d, inverse_lerp(a, b, value));
}

struct Rect {
    min: vec2f,
    max: vec2f,
}

fn toroidal_displacement(bounds: Rect, a: vec2f, b: vec2f) -> vec2f {
    let width = bounds.max.x - bounds.min.x;
    let height = bounds.max.y - bounds.min.y;

    var dx = b.x - a.x;
    var dy = b.y - a.y;

    // Adjust x displacement for wrapping
    if dx > width / 2.0 {
        dx -= width;
    } else if dx < -width / 2.0 {
        dx += width;
    }

    // Adjust y displacement for wrapping
    if dy > height / 2.0 {
        dy -= height;
    } else if dy < -height / 2.0 {
        dy += height;
    }

    return vec2f(dx, dy);
}

fn toroidal_wrap(bounds: Rect, pos: vec2f) -> vec2f {
    let width = bounds.max.x - bounds.min.x;
    let height = bounds.max.y - bounds.min.y;

    // Normalize position relative to bounds origin
    let relative_x = pos.x - bounds.min.x;
    let relative_y = pos.y - bounds.min.y;

    // Use modulo operation to wrap coordinates
    let wrapped_x = relative_x - floor(relative_x / width) * width;
    let wrapped_y = relative_y - floor(relative_y / height) * height;

    // Convert back to absolute coordinates
    return vec2f(
        wrapped_x + bounds.min.x,
        wrapped_y + bounds.min.y
    );
}

fn safe_normalize(v: vec2<f32>) -> vec2<f32> {
    let len = length(v);
    return select(v / len, vec2f(0.0), len == 0.0);
}
