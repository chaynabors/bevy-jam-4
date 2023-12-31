// Star Nest by Pablo RomÃ¡n Andrioli modified by Chay Nabors
// originally copied from https://www.shadertoy.com/view/XlfGRj
//
// This content is under the MIT License.

#import bevy_pbr::{
    mesh_view_bindings::globals,
    forward_io::VertexOutput
}

struct Material {
    player_position: vec2<f32>,
};

@group(1) @binding(0) var<uniform> material: Material;

const ITERATIONS: i32 = 15;
const FORMUPARAM: f32 = 0.53;

const VOLSTEPS: i32 = 5;
const STEPSIZE: f32 = 0.1;

const ZOOM: f32 = 2.000;
const TILE: f32 = 0.850;
const SPEED: f32 = 0.010;

const BRIGHTNESS: f32 = 0.0015;
const DARKMATTER: f32 = 0.300;
const DISTFADING: f32 = 0.730;
const SATURATION: f32 = 0.250;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
	// get coords and direction
	var rd = vec3(mesh.uv * ZOOM, 1.0);
	var ro = vec3(material.player_position, globals.time * 0.5) * 0.001 + vec3(1000.0, 500.0, 0.0);

	//volumetric rendering
	var s = 0.1;
    var fade = 1.0;
	var v = vec3(0.0);
	for (var r = 0; r < VOLSTEPS; r += 1) {
		var p = ro + s * rd * 0.5;
		p = abs(vec3(TILE) - fract(p / vec3(TILE * 2.0)) * vec3(TILE * 2.0)); // tiling fold
		var pa = 0.0;
        var a = 0.0;
		for (var i = 0; i < ITERATIONS; i += 1) {
			p = abs(p) / dot(p, p) - FORMUPARAM; // the magic formula
			a += abs(length(p) - pa); // absolute sum of average change
			pa = length(p);
		}
		let dm = max(0.0, DARKMATTER - a * a * 0.001); //dark matter
		a *= a * a; // add contrast
		if r > 6 {
            fade *= 1.0 - dm; // dark matter, don't render near
        }

		v += fade;
		v += vec3(s, s * s, s * s * s * s) * a * BRIGHTNESS * fade; // coloring based on distance
		fade *= DISTFADING; // distance fading
		s += STEPSIZE;
	}
	v = mix(vec3(length(v)), v, SATURATION); //color adjust

	return vec4(pow(vec3(v * 0.005), vec3(2.2)), 1.0);
}
