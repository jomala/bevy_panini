// This shader computes the Panini projection distortion effect as a post processing step.

// Since post processing is a fullscreen effect, we use the fullscreen vertex shader provided by bevy.
// This will import a vertex shader that renders a single fullscreen triangle.
//
// A fullscreen triangle is a single triangle that covers the entire screen.
// The box in the top left in that diagram is the screen. The 4 x are the corner of the screen
//
// Y axis
//  1 |  x-----x......
//  0 |  |  s  |  . ´
// -1 |  x_____x´
// -2 |  :  .´
// -3 |  :´
//    +---------------  X axis
//      -1  0  1  2  3
//
// As you can see, the triangle ends up bigger than the screen.
//
// You don't need to worry about this too much since bevy will compute the correct UVs for you.
#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
struct PaniniShaderSettings {
    panini: f32,
    fov_x_radians: f32,
    aspect_ratio: f32,
    compression: f32,
    // #ifdef SIXTEEN_BYTE_ALIGNMENT
    // WebGL2 structs must be 16 byte aligned.
    // #endif
}
@group(0) @binding(2) var<uniform> settings: PaniniShaderSettings;

/// My fresh implementation of the Panini inverse mapping projection with compression from the original article.
fn my_get_panini_uv_with_compression(
    uv: vec2<f32>,
) -> vec2<f32> {
    // Calculate the distance from the camera to the perspective projection plane based on the horizontal field of view.
    let half_fov: f32 = settings.fov_x_radians * 0.5;
    let cam_z: f32 = 1.0 / tan(half_fov);

    // Calculate the distance further back of the reprojection point from the plane.
    let cyl_r: f32 = 1.0 / sin(half_fov);
    let cyl_r_sq: f32 = cyl_r * cyl_r;
    let reproj_z: f32 = cyl_r * settings.panini + cam_z;
    let reproj_retreat: f32 = reproj_z - cam_z;
    let reproj_retreat_sq: f32 = reproj_retreat * reproj_retreat;

    // Convert incoming UV coordinates from both range [0, 1] to range [-1, 1] for x and [-1/aspect, 1/aspect] for y to match real space.
    let reproj_x: f32 = (uv.x * 2.0) - 1.0;
    let reproj_x_sq: f32 = reproj_x * reproj_x;
    let reproj_y: f32 = ((uv.y * 2.0) - 1.0) / settings.aspect_ratio;

    // Solve Horizontal Remapping Equation
    // This comes from solving the quadratic for the first two equations in the check below and taking the larger root.
    let reproj_xzlen_sq: f32 = reproj_z * reproj_z + reproj_x * reproj_x;
    let cyl_z: f32 = (-reproj_retreat * reproj_x_sq + reproj_z * sqrt(reproj_xzlen_sq * cyl_r_sq - reproj_retreat_sq * reproj_x_sq)) / reproj_xzlen_sq;
    let cyl_x: f32 = (cyl_z + reproj_retreat) / reproj_z * reproj_x;

    // Work out the camera plane position
    let cam_x: f32 = cyl_x / cyl_z * cam_z;

    // Solve Vertical Remapping Equation (Calculate y_r)
    // "Hard compression" can be applied here to cam_y by multiplying between 1.0 and cyl_z/cyl_r depending on a configurable factor.
    // This straightens lines but results in more visible distortion on radial lines or in .
    let vert_compression = settings.compression * (cyl_z / cyl_r) + (1.0 - settings.compression);
    let cyl_y: f32 = (cyl_z + reproj_retreat) / reproj_z * reproj_y * vert_compression;
    var cam_y: f32 = cyl_y / cyl_z * cam_z;
    
    // Transform back from range [-1, 1] for x and and [-1/aspect, 1/aspect] for y to range [0, 1] UV space
    return vec2<f32>((cam_x + 1.0) * 0.5, (cam_y * settings.aspect_ratio + 1.0) * 0.5);
}

/// Fragment shader function
@fragment
fn fragment(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    // Get the corrected texture coordinate
    // let sample_uv = get_panini_uv(uv);
    let sample_uv = my_get_panini_uv_with_compression(uv);

    // Clamping prevention: If the math samples outside the screen space, return black boundary
    if (sample_uv.x < 0.0 || sample_uv.x > 1.0 || sample_uv.y < 0.0 || sample_uv.y > 1.0) {
        return vec4<f32>(0.0, 0.0, 0.0, 1.0); 
    }

    // Sample the source frame rendering
    return textureSample(screen_texture, texture_sampler, sample_uv);
}
