#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::utils

@group(1) @binding(0)
var texture: texture_2d<f32>;

@group(1) @binding(1)
var our_sampler: sampler;

fn aa_step(compValue: f32, gradient: f32) -> f32
{
    let halfChange = fwidth(gradient) * 0.5f;

    let lowerEdge = compValue - halfChange;
    let upperEdge = compValue + halfChange;

    return( clamp((gradient - lowerEdge) / (upperEdge - lowerEdge), 0.0f, 1.0f) );
}

@fragment
fn fragment(
    @builtin(position) position: vec4<f32>,
    #import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {
    // Get screen position with coordinates from 0 to 1
    let uv = coords_to_viewport_uv(position.xy, view.viewport);
    let offset_strength = 0.001;
    let lines = 10.0;
    let line_distance = 1.0;
    let line_thickness = 0.3;

    // Sample each color channel with an arbitrary shift
    var output_color = vec4<f32>(
        textureSample(texture, our_sampler, uv + vec2<f32>(offset_strength, -offset_strength)).r,
        textureSample(texture, our_sampler, uv + vec2<f32>(-offset_strength, 0.0)).g,
        textureSample(texture, our_sampler, uv + vec2<f32>(0.0, offset_strength)).b,
        1.0
        );

    let distance = fract(position.y * 0.01 * lines);
    let dist_change = fwidth(distance);

    let intensity = smoothstep(dist_change, -dist_change, distance);//aa_step(0.5, distance);//smoothstep(0.5, 1.0, fract(position.y * 0.0001 * lines));

    // add 0.5 before fract so if dist=4 and line_distance=1 we would have 0, but in that way we will have 0.5 - we shift to the middle
    // next if we had 4.4 + 0.5 => 0.9 - 0.5 = 0.4, and if 4.3 + 0.5 => 0.8 - 0.5 = 0.3 - we are climbing but past the middle point 4.6, 4.7 => 0.4, 0.3 we are descending
    // that way where we have always 0 where the line should be, but it will have smooth step from left and right.
    // we multiply it back by line_distance to have the proper scaling and not the values clamped between 0 and 1.
    let majorLineDistance = abs(fract(distance / line_distance + 0.5) - 0.5) * line_distance;
    let majorLines = smoothstep(line_thickness - dist_change * 5.0, line_thickness + dist_change * 5.0, majorLineDistance);

    let intensity = majorLines;

    var new_output_color = vec4<f32>(
        textureSample(texture, our_sampler, uv).x * intensity,
        textureSample(texture, our_sampler, uv).y * intensity,
        textureSample(texture, our_sampler, uv).z * intensity,
        1.0
    );

    return new_output_color;
}
