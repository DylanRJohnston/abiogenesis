#define_import_path utils::colours

const RED: vec3<f32> = vec3<f32>(172.0 / 255.0, 40.0 / 255.0, 71.0 / 255.0);
const GREEN: vec3<f32> = vec3<f32>(90.0 / 255.0, 181.0 / 255.0, 82.0 / 255.0);
const BLUE: vec3<f32> = vec3<f32>(51.0 / 255.0, 136.0 / 255.0, 222.0 / 255.0);
const ORANGE: vec3<f32> = vec3<f32>(255.0 / 255.0, 155.0 / 255.0, 37.0 / 255.0);
const PINK: vec3<f32> = vec3<f32>(233.0 / 255.0, 75.0 / 255.0, 234.0 / 255.0);
const AQUA: vec3<f32> = vec3<f32>(57.0 / 255.0, 247.0 / 255.0, 241.0 / 255.0);

const COLOURS: array<vec3<f32>, NUM_COLOURS> = array<vec3<f32>, NUM_COLOURS>(RED, GREEN, BLUE, ORANGE, PINK, AQUA);

const NUM_COLOURS: u32 = 6;

const RED_ID: u32 = 0;
const GREEN_ID: u32 = 1;
const BLUE_ID: u32 = 2;
const ORANGE_ID: u32 = 3;
const PINK_ID: u32 = 4;
const AQUA_ID: u32 = 5;

const EMPTY_ID: u32 = 0 << 1;
