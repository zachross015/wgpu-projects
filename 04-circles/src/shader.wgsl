@group(0)
@binding(0)
var<uniform> resolution: vec2f;

struct VertexOutput {
	@builtin(position) position: vec4f,
	@location(1) center: vec2f,
	@location(2) radius: f32
};


@vertex
fn vs_main(@location(0) vertex_position: vec2f, @location(1) center: vec2f, @location(2) radius: f32) -> VertexOutput {
	var output: VertexOutput;
	let center_position = 2.0 * ((center / resolution) - 1.0);
	let radius_resolution = radius / resolution;
	output.position = vec4f((vertex_position + center_position) * radius_resolution, 1.0, 4.0);
	output.radius = radius;
	output.center = center;
	return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4f {

	// let center = resolution * input.center;

	// if(distance(input.position.xy, center) > input.radius) {
	// 	discard;
	// }

	if(input.position.y < resolution.y / 2.0)  {
		return vec4f(1.0, 0.0, 0.0, 1.0);
	}

	return vec4f(vec3f(1.0), 1.0);
}

