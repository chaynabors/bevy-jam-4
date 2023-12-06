#import bevy_pbr::{
    mesh_view_bindings::globals,
    forward_io::VertexOutput
}

struct LineMaterial {
    time: f32,
};

@group(1) @binding(0) var<uniform> material: LineMaterial;

const ITERATIONS: i32 = 12;
const FORMUPARAM: f32 = 0.53;

const VOLSTEPS: i32 = 20;
const STEPSIZE: f32 = 0.1;

const ZOOM: f32 = 2.000;
const TILE: f32 = 0.850;
const SPEED: f32 = 0.010;

const BRIGHTNESS: f32 = 0.0015;
const DARKMATTER: f32 = 0.300;
const DISTFADING: f32 = 0.730;
const SATURATION: f32 = 0.850;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
	// get coords and direction
	var rd = vec3(mesh.uv * ZOOM, 1.0);
	let time = globals.time * SPEED * 0.25;

	var ro = vec3(1.0, 0.5, 0.5);
	ro += vec3(time * 2.0, time, -2.0);

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

	return vec4(pow(vec3(v * 0.01), vec3(2.2)), 1.0);
}
