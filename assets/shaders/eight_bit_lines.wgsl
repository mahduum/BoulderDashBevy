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

//compValue is where we want to place the anti aliased line, for 0.5 it will be in the middle (0,1)
//fn aaStep(in float compValue, in float gradient){//gradient is our value of a line between 0 and 1 for frac uv.y multiplied by lines
//  float halfChange = fwidth(gradient) * 0.5f;//change will be constant, because it is vertical, it will somehow depend on smoothstep
//  //base the range of the inverse lerp on the change over one pixel
//  float lowerEdge = compValue - halfChange;//values of the lower edge and respectively upper edge
//  float upperEdge = compValue + halfChange;
//  //do the inverse interpolation
//  return( clamp((gradient - lowerEdge) / (upperEdge - lowerEdge), 0.0f, 1.0f) );//the value of the gradient at current frac(uv.y * lines)
// for example lower edge 0.3, upper edge 0.7 if gradient is 0.4, then 0.1/0.4, if gradient is 0.5 then 0.2/0.4 (1/2), if 0.7 then 0.4/0.4 (1)
//}

//draw on the line but line is shifted
