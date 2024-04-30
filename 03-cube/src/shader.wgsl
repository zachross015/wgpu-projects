struct VertexOutput {
	@builtin(position) position: vec4<f32>,
	@location(0) color: vec4<f32>
}


@group(0)
@binding(0)
var<uniform> time: f32;


@vertex
fn vs_main(@location(0) position: vec4<f32>, @location(1) color: vec4<f32>) -> VertexOutput {

	var s = sin(time);
	var c = cos(time);
	var yaw_matrix = mat4x4(
		vec4(c, -s, 0, 0),
		vec4(s, c, 0, 0),
		vec4(0, 0, 1, 0),
		vec4(0, 0, 0, 1),
	);

	var pitch_matrix = mat4x4(
		vec4(c, 0, s, 0),
		vec4(0, 1, 0, 0),
		vec4(-s, 0, c, 0),
		vec4(0, 0, 0, 1),
	);

	var roll_matrix = mat4x4(
		vec4(1, 0, 0, 0),
		vec4(0, c, -s, 0),
		vec4(0, s, c, 0),
		vec4(0, 0, 0, 1),
	);


	var output: VertexOutput;
	output.position = yaw_matrix * pitch_matrix * roll_matrix * position;
	output.color = color;

	output.position.w = 4.0;


	return output;
}


@fragment
fn fs_main(vertex: VertexOutput) -> @location(0) vec4<f32> {
	return vertex.color;
}
