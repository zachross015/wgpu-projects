@group(0)
@binding(0)
var<uniform> center: vec2f;


struct VertexOutput {
	@builtin(position) position: vec4f,
};


@vertex
fn vs_main(@location(0) position: vec2f) -> VertexOutput {
	var output: VertexOutput;
	output.position = vec4f(position, 1.0, 4.0);
	return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4f {

	let radius = 100.0;

	if(distance(input.position.xy, center) > radius) {
		discard;
	}

	return vec4f(vec3f(1.0), 1.0);
}

