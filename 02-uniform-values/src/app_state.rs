use encase::ShaderType;

const ZOOM_INCREMENT_FACTOR: f32 = 1.1;
const CAMERA_POS_INCREMENT_FACTOR: f32 = 0.1;


// Uniform to be sent to the shader
#[derive(ShaderType)]
pub struct AppState {
    pub cursor_pos:  glam::Vec2,
    pub zoom: f32,
    pub max_iterations: u32,
}

impl AppState {
    pub fn as_wgsl_bytes(&self) -> encase::internal::Result<Vec<u8>> {
        let mut buffer = encase::UniformBuffer::new(vec![]); 
        buffer.write(self)?;
        Ok(buffer.into_inner())
    }

    pub fn translate_view(&mut self, increments: i32, axis: usize) {
        self.cursor_pos[axis] += CAMERA_POS_INCREMENT_FACTOR * increments as f32 / self.zoom;
    }

    pub fn zoom(&mut self, amount: f32) {
        self.zoom += ZOOM_INCREMENT_FACTOR * amount * self.zoom.powf(1.02);
        self.zoom = self.zoom.max(1.1);
    }
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            cursor_pos: glam::Vec2::ZERO,
            zoom: 1.0,
            max_iterations: 50
        }
    }
}
