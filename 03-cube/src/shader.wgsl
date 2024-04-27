struct VertexOutput {
	@builtin(position) position: vec4<f32>,
	@location(0) color: vec4<f32>
}


@group(0)
@binding(0)
var<uniform> time: f32;


@vertex
fn vs_main(@location(0) position: vec4<f32>, @location(1) color: vec4<f32>) -> VertexOutput {
	var output: VertexOutput;
	output.position = position;
	output.color = color;

	output.position.x -= 0.5;
	output.position.y -= 0.5;
	output.position.w = 4.0;

	output.position.y *= sin(time);


	return output;
}


@fragment
fn fs_main(vertex: VertexOutput) -> @location(0) vec4<f32> {
	return vertex.color;
}
