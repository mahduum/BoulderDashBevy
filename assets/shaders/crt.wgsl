#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::utils

@group(1) @binding(0)
var texture: texture_2d<f32>;

@group(1) @binding(1)
var our_sampler: sampler;

//@group(1) @binding(2)
//var tile_texture: texture_2d<f32>;

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

    let _Density = 49;//todo with 15 there are big gaps in horizontal lines
    let _VertsColor = 0.5;
    let _VertsColor2 = 0.2;
    let _ScansColor = vec4(0.2, 0.4, 0.6, 1.0);
    let _Br = 0;
    let _Contrast = 0.0;

    // Get screen position with coordinates from 0 to 1
    let uv = coords_to_viewport_uv(position.xy, view.viewport);
    let screen_height = i32(view.viewport.w);
    let lines = 30;
    //how many pixels: every pixel is 2 pixels and is divided by scale
    //how many pixels screen has, what is the size in pixels of the tile, tile size is 32 pixels
    let pixel_width = 7;//i32(2.0/*fake pixel true size*/ * 0.005);
    ///Curvature
    ///You only need a distortion texture, that you can ignore at first. The main texture is the camera output. This is an Image Effect / Post Processing shader
    // half2 n = tex2D(_DisplacementTex, i.uv);
    // half2 d = n * 2 - 1;//change the domain to be between -1 and 1
    // i.uv += d * _Strength;
    // i.uv = saturate(i.uv);
    // ///

    // //Distort image on y axis
    // i.uv.y += _Distort;
    //sample the main texture
    let color =
        textureSample(texture, our_sampler, uv.xy);
    
    //Vertical lines
    let ps = position.xy * view.viewport.zw / position.w;//_ScreenParams.xy - camera target's width and height, scr_pos is in 0,0 to 1,1, corrected by perspective divide w for the output so it has "perspective look" in 2d screen space
    let psx: i32 = i32(ps.x);
    let pp = psx % _Density;//is always within _Density
    var outcolor = vec4<f32>(0.0, 0.0, 0.0, 1.0);
    let muls = vec4<f32>(0.0, 0.0, 0.0, 1.0);
    if (pp < _Density/3){//if it's less then 1/3 of the _Density modify channels g and b
        outcolor.r = color.r;
        outcolor.g = color.g*_VertsColor; 
        outcolor.b = color.b*_VertsColor2; 
    }
    else if (pp < (2*_Density)/3){//then if it's less then 2/3 of the _Density modify channels r and b
        outcolor.g = color.g;
        outcolor.r = color.r*_VertsColor;
        outcolor.b = color.b*_VertsColor2;
    }
    else{//for the last third part of the _Density modify channels r and g
        outcolor.b = color.b;
        outcolor.r = color.r*_VertsColor;
        outcolor.g = color.g*_VertsColor2; 
    }

    //how many pixels viewport has, then how high is the tile, and how high is 1/16 of the tile: this will be the pixel size.
    //given the tile is 32 pixels, and one pixel is 2, then vie
    let half_viewport = i32(view.viewport.w/2.0);//todo NOTE: viewport scale is different than window scale
    let dim: vec2<i32> = textureDimensions(texture, 0);//viewport height == textureDimensions()
    let scale = view.viewport.w / 1024.0;
    let pixel_size = i32(8.0/scale);
    //Horizontal lines
    //Modify all colors but only on the exact _Density step (in pixels?)
    //take the viewport height, get uv screen coordinates, divide and set line on equal % frequency
    let psy: i32 = i32(ps.y);
    //if (psy % _Density == 0) {
    //if (uv.y % _Density == 0) {
    //if (i32(position.y) % pixel_size == 0) {
    if ((i32(position.y) * 1000) % 64 == 0) {
        outcolor *= vec4<f32>(_ScansColor.r, _ScansColor.g, _ScansColor.b, 1.0);
    }

    //Color correciton
    //outcolor += (_Br / 255);//add brightness//todo doesn't work because types
    outcolor = outcolor - _Contrast * (outcolor - 1.0) * outcolor *(outcolor - 0.5);//it will keep the values in the middle the same, increase the values > 0.5, and decrease when < 0.5

    //Scan lines
    //_ScanPoint and _Distort are dynamically modified on material from code in OnRenderImage
    // if ((position.y *view.viewport.w) >= _ScanPoint && (position.y *view.viewport.w) < _ScanPoint + _ScanThikness)
    // {
    //     outcolor *= _ScanDensity;
    // }

    return outcolor;
}
