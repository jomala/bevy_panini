#import bevy_pbr::forward_io::VertexOutput

@fragment
fn fragment(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
    // 1. Grab the 3D position coordinates of this pixel fragment
    let world_pos: vec3<f32> = in.world_position.xyz;

    // 2. Create a grid line pattern by checking the distance to the nearest whole integer
    let grid_size = 0.1;
    let line_width = 0.02;

    let coord = fract((world_pos / grid_size) + vec3<f32>(0.5, 0.0, 0.5));
    
    // Check if the fragment is close to any grid line boundary
    let grid_x = step(coord.x, line_width) + step(1.0 - line_width, coord.x);
    let grid_y = step(coord.y, line_width) + step(1.0 - line_width, coord.y);
    let grid_z = step(coord.z, line_width) + step(1.0 - line_width, coord.z);

    // Combine axes so lines show on all faces of the 3D object
    let is_grid_line = max(max(grid_x, grid_y), grid_z);

    // 3. Color the object based on its actual position + grid lines
    // Base color matches the world coordinates (X=Red, Y=Green, Z=Blue)
    let final_color = mix(vec3<f32>(0.9, 0.9, 0.2), vec3<f32>(0.2, 0.2, 0.2), is_grid_line);

    return vec4<f32>(final_color, 1.0);
}
