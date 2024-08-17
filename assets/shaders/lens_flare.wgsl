// /// ***************************** ///
// /// THIS IS THE DEFAULT 2D SHADER ///
// /// You can always get back to this with `python3 scripts/reset-2d.py` ///
// /// ***************************** ///

#import bevy_sprite::mesh2d_view_bindings::globals 
#import shadplay::shader_utils::common::{intoPolar, PI, NEG_HALF_PI, shader_toy_default, rotate2D, TWO_PI}
#import bevy_render::view::View
#import bevy_pbr::forward_io::VertexOutput;

@group(0) @binding(0) var<uniform> view: View;

@group(2) @binding(0) var<uniform> shdr: LensFlareShader;

@group(2) @binding(1) var texture: texture_2d<f32>;

@group(2) @binding(2) var texture_sampler: sampler;

struct LensFlareShader{
    center_pos : vec2f,
}

const SPEED:f32 = 1.0;

fn cc(color: vec4<f32>, factor: f32, factor2: f32) -> vec4<f32> {
	let w: f32 = color.x + color.y + color.z;
	return mix(color, vec4<f32>(w) * factor, w * factor2);
} 


// Fragment shader
@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
     // Define parameters for the lens flare effect
    let resolution = view.viewport.zw;
    var lightSource: vec2<f32> = vec2<f32>(1. - 0. * 1.5, - 1. );  // Position of the light source in normalized coordinates
    let flareIntensity: f32 = 2.0;                             // Intensity of the lens flare effect
    let haloSize: f32 = 30.* (sin(globals.time) * 1.2 + 1.7);  // Size of the halo effect
    let st = in.uv * 2. - 1.;

    let uv = st * resolution.y / resolution.x;

    lightSource *= resolution.y / resolution.x;

    // Calculate vector from the light source to the current pixel
    var lightDir: vec2<f32> = uv - lightSource;

    // Normalize the light direction vector
    // lightDir = normalize(lightDir);

    var main: vec2<f32> = uv - lightSource;
    var uvd: vec2<f32> = uv * length(uv);

    var ang: f32 = atan2(lightDir.x, lightDir.y);
    ang = ang + (sin(globals.time) / 15. -0.5); 
    var dist: f32 = length(lightDir);
    dist = pow(dist, 0.1);

    // var f0: f32 = 1.0 / (length(uv  - lightSource) * haloSize + 0.1);
    // f0 = f0 + f0 * (sin(
    //     sin(ang * 2.5 + lightSource.x) * 0.10 - cos(ang * 2.5 + lightSource.y) * 16.0 )
    //     * 0.02 + dist * 0.6 
    // );

    // f0 = f0 - textureSample(texture, texture_sampler,  uv).x * 0.015;


    // var f1: f32 = max(0.01 - pow(length(uv + 1.2 * lightSource), 1.9), 0.0) * 7.0;

    var f0: f32 = max(0., smoothstep((sin(globals.time) *0.03 + 0.55), 1.2, abs(st.x)));
    // var f01: f32 = max(0., (smoothstep(-1., -0.54, uv.x) - smoothstep(-0.85, -0.54, uv.x)) );
    var f02: f32 = max(0., smoothstep((sin(globals.time) *0.03 + 0.6) , 1.1, abs(st.y)));
    // var f03: f32 = max(0., smoothstep(-1., -0.54, uv.y) - smoothstep(-0.85, -0.54, uv.y));

    let border_color = vec4f(f0 +  f02 ); 

    // var f2: f32 = max(1.0 / (1.0 + 32.0 * pow(length(uvd + 0.70 * lightSource), 2.0)), 0.0) * 0.25;
    // var f22: f32 = max(1.0 / (1.0 + 32.0 * pow(length(uvd + 0.75 * lightSource), 2.0)), 0.0) * 0.23;
    // var f23: f32 = max(1.0 / (1.0 + 32.0 * pow(length(uvd + 0.8 * lightSource), 2.0)), 0.0) * 0.21;

    // var uvx: vec2<f32> = mix(uv, uvd, -0.5);

    // var f4: f32 = max(0.01 - pow(length(uvx + 0.4 * lightSource), 2.4), 0.0) * 6.0;
    // var f42: f32 = max(0.01 - pow(length(uvx + 0.45 * lightSource), 2.4), 0.0) * 5.0;
    // var f43: f32 = max(0.01 - pow(length(uvx + 0.5 * lightSource), 2.4), 0.0) * 3.0;


    // uvx = mix(uv, uvd, -0.4);

    // var f5: f32 = max(0.01 - pow(length(uvx + 0.1 * lightSource), 5.5), 0.0) * 2.0;
    // var f52: f32 = max(0.01 - pow(length(uvx + 0.3 * lightSource), 5.5), 0.0) * 2.0;
    // var f53: f32 = max(0.01 - pow(length(uvx + 0.5 * lightSource), 5.5), 0.0) * 2.0;

    // uvx = mix(uv, uvd, -0.5);

    // var f6 = 0.;
    // var f62 = 0.;
    // var f63  = 0.;

    // f6 = max(0.01 - pow(length(uvx - 0.3 * lightSource), 1.6), 0.0) * 6.0;
    // f62 = max(0.01 - pow(length(uvx - 0.325 * lightSource), 1.6), 0.0) * 3.0;
    // f63 = max(0.01 - pow(length(uvx - 0.35 * lightSource), 1.6), 0.0) * 5.0;

    // Calculate the ghost artifacts
    // var ghost1: vec4<f32> = textureSample(texture, texture_sampler,  uv  + lightDir * ghostSpacing * ghostDispersal);
    // var ghost2: vec4<f32> = textureSample(texture, texture_sampler,  uv  + lightDir * ghostSpacing * ghostDispersal * 2.0);
    // var ghost3: vec4<f32> = textureSample(texture, texture_sampler,  uv  + lightDir * ghostSpacing * ghostDispersal * 3.0);

    // Combine the lens flare components with the original texture
    var flareColor: vec4<f32> = textureSample(texture, texture_sampler, in.uv);
    
    // flareColor.r +=  f4 + f5 + f6;
    // flareColor.g += f42 + f52 + f62;
    // flareColor.b +=  f43 + f53 + f63;
    // flareColor.a += min( f43 + f53 + f63  + f42 + f52 + f62  + f4 + f5 + f6, 1.);

    // flareColor = flareColor * 3. - vec4f(length(uvd)*.15);

    // flareColor += f0 * flareIntensity * vec4<f32>(1.4, 1.2, 1., 1.) ;

    // flareColor = flareColor - textureSample(texture, texture_sampler,  uv) * 0.015;

    // flareColor = cc(flareColor, 0.5, 0.1);

    flareColor += border_color;

    // flareColor.a = 1.;

    // Output the final color with the lens flare effect applied
    return flareColor*1.7;
}