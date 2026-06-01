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
struct PaniniSettings {
    panini: f32,
    fov_x_radians: f32,
    aspect_ratio: f32,
#ifdef SIXTEEN_BYTE_ALIGNMENT
    // WebGL2 structs must be 16 byte aligned.
    _webgl2_padding: f32
#endif
}
@group(0) @binding(2) var<uniform> settings: PaniniSettings;

// Main Panini Inverse Mapping Function
fn get_panini_uv(uv: vec2<f32>) -> vec2<f32> {
    // 1. Convert incoming UV coordinate from [0, 1] to centered [-1, 1]
    let h_v: vec2<f32> = (uv * 2.0) - vec2<f32>(1.0);

    // 2. Calculate the maximum bounds to automatically fit the screen edges
    // This scales the math to ensure the left/right screen edges match perfectly.
    let half_fov: f32 = settings.fov_x_radians * 0.5;
    let max_h: f32 = ((settings.panini + 1.0) * sin(half_fov)) / (settings.panini + cos(half_fov));
    
    // Scale our current horizontal point by the maximum boundary scale
    let h: f32 = h_v.x * max_h;

    // 3. Solve Horizontal Remapping Equation (Calculate x_r)
    let d_plus_1: f32 = settings.panini + 1.0;
    let d_sqr: f32 = settings.panini * settings.panini;
    let denom_factor: f32 = 1.0 - (d_sqr / (d_plus_1 * d_plus_1));
    let gamma: f32 = sqrt(1.0 + (h * h * denom_factor));
    
    let x_r: f32 = (h * d_plus_1) / (d_plus_1 - (settings.panini * gamma));

    // 4. Solve Vertical Remapping Equation (Calculate y_r)
    let cos_phi: f32 = 1.0 / sqrt(1.0 + (x_r * x_r));
    
    // Un-scale the vertical axis using the maximum edge constraint and aspect ratio
    let max_v: f32 = h_v.y * (tan(half_fov) / settings.aspect_ratio);
    let y_r: f32 = max_v * ((settings.panini + cos_phi) / d_plus_1);

    // 5. Re-apply the initial camera projection scale to safely fit back into texture space
    let final_x: f32 = x_r / tan(half_fov);
    let final_y: f32 = (y_r * settings.aspect_ratio) / tan(half_fov);

    // Transform back from centered [-1, 1] space to normal [0, 1] UV space
    return (vec2<f32>(final_x, final_y) + vec2<f32>(1.0)) * 0.5;
}

@fragment
fn fragment(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    // Get the corrected texture coordinate
    let sample_uv = get_panini_uv(uv);

    // Clamping prevention: If the math samples outside the screen space, return black boundary
    if (sample_uv.x < 0.0 || sample_uv.x > 1.0 || sample_uv.y < 0.0 || sample_uv.y > 1.0) {
        return vec4<f32>(0.0, 0.0, 0.0, 1.0); 
    }

    // Sample the source frame rendering
    return textureSample(screen_texture, texture_sampler, sample_uv);
}
