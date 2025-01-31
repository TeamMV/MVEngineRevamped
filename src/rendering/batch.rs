use gl::types::GLuint;
use crate::rendering::{PrimitiveRenderer, Quad, Triangle};
use crate::rendering::camera::OrthographicCamera;
use crate::rendering::shader::OpenGLShader;
use crate::window::Window;

pub const BATCH_VERTEX_AMOUNT: usize = 1_000;

pub const POSITION_SIZE: usize = 3;
pub const POSITION_SIZE_BYTES: usize = POSITION_SIZE * 4;
pub const POSITION_OFFSET: usize = 0;
pub const POSITION_OFFSET_BYTES: usize = POSITION_OFFSET * 4;

pub const COLOR_SIZE: usize = 4;
pub const COLOR_SIZE_BYTES: usize = COLOR_SIZE * 4;
pub const COLOR_OFFSET: usize = POSITION_OFFSET + POSITION_SIZE;
pub const COLOR_OFFSET_BYTES: usize = COLOR_OFFSET * 4;

pub const UV_SIZE: usize = 2;
pub const UV_SIZE_BYTES: usize = UV_SIZE * 4;
pub const UV_OFFSET: usize = COLOR_OFFSET + COLOR_SIZE;
pub const UV_OFFSET_BYTES: usize = UV_OFFSET * 4;

pub const TEXTURE_SIZE: usize = 1;
pub const TEXTURE_SIZE_BYTES: usize = TEXTURE_SIZE * 4;
pub const TEXTURE_OFFSET: usize = UV_OFFSET + UV_SIZE;
pub const TEXTURE_OFFSET_BYTES: usize = TEXTURE_OFFSET * 4;

pub const VERTEX_SIZE: usize = POSITION_SIZE + COLOR_SIZE + UV_SIZE + TEXTURE_SIZE;
pub const VERTEX_SIZE_BYTES: usize = VERTEX_SIZE * 4;

pub const MAX_TEXTURES: usize = 16;

pub(crate) struct RenderBatch {
    pub(crate) vertex_data: [f32; VERTEX_SIZE * BATCH_VERTEX_AMOUNT],
    pub(crate) index_data: [u32; BATCH_VERTEX_AMOUNT * 6],
    pub(crate) texture_data: [f32; MAX_TEXTURES],
    vertex_index: usize,
    index_index: usize,
    texture_index: usize,
    triangle_index: usize,
    vbo_id: GLuint,
    ibo_id: GLuint,
    shader: OpenGLShader
}

impl RenderBatch {
    pub(crate) unsafe fn new(shader: OpenGLShader) -> Self {
        let mut vbo_id = 0;
        let mut ibo_id = 0;
        gl::GenBuffers(1, &mut vbo_id);
        gl::GenBuffers(1, &mut ibo_id);

        let mut texture_units = 0;
        gl::GetIntegerv(gl::MAX_TEXTURE_IMAGE_UNITS, &mut texture_units);

        Self {
            vertex_data: [0.0; VERTEX_SIZE * BATCH_VERTEX_AMOUNT],
            index_data: [0; BATCH_VERTEX_AMOUNT * 6],
            texture_data: [0.0; MAX_TEXTURES],
            vertex_index: 0,
            index_index: 0,
            texture_index: 0,
            triangle_index: 0,
            vbo_id,
            ibo_id,
            shader,
        }
    }

    pub(crate) fn push_triangle(&mut self, triangle: Triangle) {
        for vertex in triangle.points {
            self.vertex_data[self.vertex_index + 0] = vertex.pos.0 as f32;
            self.vertex_data[self.vertex_index + 1] = vertex.pos.1 as f32;
            self.vertex_data[self.vertex_index + 2] = vertex.pos.2 as f32;

            let col_vec = vertex.color.as_vec4();
            self.vertex_data[self.vertex_index + 3] = col_vec.x;
            self.vertex_data[self.vertex_index + 4] = col_vec.y;
            self.vertex_data[self.vertex_index + 5] = col_vec.z;
            self.vertex_data[self.vertex_index + 6] = col_vec.w;

            self.vertex_data[self.vertex_index + 7] = vertex.uv.0;
            self.vertex_data[self.vertex_index + 8] = vertex.uv.1;

            self.vertex_data[self.vertex_index + 9] = vertex.texture as f32;
            self.vertex_index += VERTEX_SIZE;
        }

        self.index_data[self.index_index + 0] = self.triangle_index as u32 + 0;
        self.index_data[self.index_index + 1] = self.triangle_index as u32 + 1;
        self.index_data[self.index_index + 2] = self.triangle_index as u32 + 2;

        self.index_index += 3;
        self.triangle_index += 1;
    }

    pub(crate) fn push_quad(&mut self, quad: Quad) {
        for vertex in quad.points {
            self.vertex_data[self.vertex_index + 0] = vertex.pos.0 as f32;
            self.vertex_data[self.vertex_index + 1] = vertex.pos.1 as f32;
            self.vertex_data[self.vertex_index + 2] = vertex.pos.2 as f32;

            let col_vec = vertex.color.as_vec4();
            self.vertex_data[self.vertex_index + 3] = col_vec.x;
            self.vertex_data[self.vertex_index + 4] = col_vec.y;
            self.vertex_data[self.vertex_index + 5] = col_vec.z;
            self.vertex_data[self.vertex_index + 6] = col_vec.w;

            self.vertex_data[self.vertex_index + 7] = vertex.uv.0;
            self.vertex_data[self.vertex_index + 8] = vertex.uv.1;

            self.vertex_data[self.vertex_index + 9] = vertex.texture as f32;
            self.vertex_index += VERTEX_SIZE;
        }

        self.index_data[self.index_index + 0] = self.triangle_index as u32 + 0;
        self.index_data[self.index_index + 1] = self.triangle_index as u32 + 1;
        self.index_data[self.index_index + 2] = self.triangle_index as u32 + 2;

        self.index_data[self.index_index + 3] = self.triangle_index as u32 + 2;
        self.index_data[self.index_index + 4] = self.triangle_index as u32 + 3;
        self.index_data[self.index_index + 5] = self.triangle_index as u32 + 0;

        self.index_index += 6;
        self.triangle_index += 2;
    }

    fn has_texture(&self, id: GLuint) -> bool {
        self.texture_data.contains(&(id as f32))
    }

    pub fn can_hold_triangle(&self, triangle: &Triangle) -> bool {
        if self.vertex_index + 3 > BATCH_VERTEX_AMOUNT { return false; }

        let mut needed_tex = 0;
        let mut seen = Vec::new();
        for vertex in &triangle.points {
            if vertex.has_texture {
                if !seen.contains(&vertex.texture) && !self.has_texture(vertex.texture) {
                    needed_tex += 1;
                    seen.push(vertex.texture);
                }
            }
        }

        if self.texture_index + needed_tex > MAX_TEXTURES {
            return false;
        }

        true
    }

    pub fn can_hold_quad(&self, quad: &Quad) -> bool {
        if self.vertex_index + 4 > BATCH_VERTEX_AMOUNT { return false; }

        let mut needed_tex = 0;
        let mut seen = Vec::new();
        for vertex in &quad.points {
            if vertex.has_texture {
                if !seen.contains(&vertex.texture) && !self.has_texture(vertex.texture) {
                    needed_tex += 1;
                    seen.push(vertex.texture);
                }
            }
        }

        if self.texture_index + needed_tex > MAX_TEXTURES {
            return false;
        }

        true
    }

    pub(crate) fn prepare_batch(&mut self) {
        self.vertex_index = 0;
        self.index_index = 0;
        self.triangle_index = 0;
        self.texture_index = 0;
    }

    pub fn is_empty(&self) -> bool {
        self.vertex_index == 0
    }

    pub fn draw(&mut self, window: &Window, camera: &OrthographicCamera, renderer: &mut impl PrimitiveRenderer) {
        renderer.draw_data(
            window,
            camera,
            &self.vertex_data,
            &self.index_data,
            &self.texture_data,
            self.vbo_id,
            self.ibo_id,
            self.triangle_index as u32 * 3,
            &mut self.shader
        );
        self.prepare_batch();
    }
}