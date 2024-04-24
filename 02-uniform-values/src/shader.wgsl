struct AppState {
	cursor_pos_x: f32,
	cursor_pos_y: f32,
	zoom: f32,
	max_iterations: u32
}


@group(0)
@binding(0)
var<uniform> state: AppState;

struct VertexOutput {
	@builtin(position) position: vec4f,
	@location(0) coord: vec2f,
}

@vertex
fn vs_main(@builtin(vertex_index) index: u32) -> VertexOutput {
	var p = vec2f(0.0, 0.0);
	if (index == 0u) {
		p = vec2f(-1, 1);
	} else if (index == 1u) {
		p = vec2f(3.0, 1.0);
	} else {
		p = vec2f(-1.0, -3.0);
	}
	var out: VertexOutput;
	out.coord = p;
	out.position = vec4f(p, 0.0, 1.0);
	return out;
}

@fragment
fn fs_main(vin: VertexOutput) -> @location(0) vec4f {
	let max_iterations = state.max_iterations;
	var final_iteration = max_iterations;

	let c = (vin.coord * 3.0 / state.zoom) + vec2(state.cursor_pos_x, state.cursor_pos_y);
	var current_z = c;
	var next_z: vec2f;
	for (var i = 0u; i < max_iterations; i++) {
		next_z.x = (current_z.x * current_z.x - current_z.y * current_z.y) + c.x;
		next_z.y = (2.0 * current_z.x * current_z.y) + c.y;
		current_z = next_z;
		if length(current_z) > 4.0 {
			final_iteration = i;
			break;
		}
	}
	let value = f32(final_iteration) / f32(max_iterations);

	return vec4(value, value, value, 1.0);
}
