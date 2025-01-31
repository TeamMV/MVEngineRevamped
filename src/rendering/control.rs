use crate::rendering::batch::RenderBatch;
use crate::rendering::shader::OpenGLShader;
use crate::rendering::{PrimitiveRenderer, Quad, Triangle};
use crate::rendering::camera::OrthographicCamera;
use crate::window::Window;

pub struct RenderController {
    default_shader: OpenGLShader,
    batches: Vec<RenderBatch>,
    batch_index: usize
}

impl RenderController {
    pub fn new(default_shader: OpenGLShader) -> Self {
        unsafe {
            Self {
                default_shader: default_shader.clone(),
                batches: vec![RenderBatch::new(default_shader)],
                batch_index: 0,
            }
        }
    }

    fn setup(&mut self) {
        unsafe {
            self.batches = vec![RenderBatch::new(self.default_shader.clone())];
            self.batch_index = 0;
        }
    }

    pub fn push_triangle(&mut self, triangle: Triangle) {
        unsafe {
            let current = &mut self.batches[self.batch_index];
            if current.can_hold_triangle(&triangle) {
                current.push_triangle(triangle);
            } else {
                self.batches.push(RenderBatch::new(self.default_shader.clone()));
                self.batch_index += 1;
                self.push_triangle(triangle);
            }
        }
    }

    pub fn push_quad(&mut self, quad: Quad) {
        unsafe {
            let current = &mut self.batches[self.batch_index];
            if current.can_hold_quad(&quad) {
                current.push_quad(quad);
            } else {
                self.batches.push(RenderBatch::new(self.default_shader.clone()));
                self.batch_index += 1;
                self.push_quad(quad);
            }
        }
    }

    pub fn draw(&mut self, window: &Window, camera: &OrthographicCamera, renderer: &mut impl PrimitiveRenderer) {
        for batch in &mut self.batches {
            if !batch.is_empty() {
                batch.draw(window, camera, renderer);
            }
        }
        self.batch_index = 0;
    }
}