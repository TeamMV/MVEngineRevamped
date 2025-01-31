use crate::color::RgbColor;
use crate::rendering::batch::RenderBatch;
use crate::window::Window;
use gl::types::{GLint, GLsizei, GLsizeiptr, GLuint};
use std::ffi::CString;
use std::num::NonZeroU32;
use std::os::raw::c_void;
use std::ptr::null;
use std::str::FromStr;
use crate::rendering::camera::OrthographicCamera;
use crate::rendering::shader::OpenGLShader;

pub mod batch;
pub mod texture;
pub mod shader;
pub mod camera;
pub mod control;

pub struct Vertex {
    pub pos: (i32, i32, i32),
    pub color: RgbColor,
    pub uv: (f32, f32),
    pub texture: GLuint,
    pub has_texture: bool
}

pub struct Triangle {
    pub points: [Vertex; 3],
}

pub struct Quad {
    pub points: [Vertex; 4],
}

pub trait PrimitiveRenderer {
    fn draw_data(&mut self, window: &Window, camera: &OrthographicCamera, vertices: &[f32], indices: &[u32], textures: &[f32], vbo: GLuint, ibo: GLuint, amount: u32, shader: &mut OpenGLShader);
}

pub struct OpenGLRenderer {

}

impl OpenGLRenderer {
    pub unsafe fn initialize(window: &Window) -> Self {
        let handle = window.get_handle();

        handle.make_current().expect("Cannot make OpenGL context current");

        Self {}
    }
}

impl PrimitiveRenderer for OpenGLRenderer {
    fn draw_data(&mut self, window: &Window, camera: &OrthographicCamera, vertices: &[f32], indices: &[u32], textures: &[f32], vbo: GLuint, ibo: GLuint, amount: u32, shader: &mut OpenGLShader) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER, vertices.len() as GLsizeiptr, vertices.as_ptr() as *const _, gl::DYNAMIC_DRAW);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, indices.len() as GLsizeiptr, indices.as_ptr() as *const _, gl::DYNAMIC_DRAW);

            shader.uniform_1f("uResX", window.info.width as f32);
            shader.uniform_1f("uResY", window.info.height as f32);
            shader.uniform_matrix_4fv("uProjection", &camera.get_projection());
            shader.uniform_matrix_4fv("uView", &camera.get_view());
            shader.uniform_1fv("TEX_SAMPLER", &textures);

            gl::VertexAttribPointer(0, batch::POSITION_SIZE as GLint, gl::FLOAT, 0, batch::VERTEX_SIZE_BYTES as GLsizei, batch::POSITION_OFFSET_BYTES as *const c_void);
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(1, batch::COLOR_SIZE as GLint, gl::FLOAT, 0, batch::VERTEX_SIZE_BYTES as GLsizei, batch::COLOR_OFFSET_BYTES as *const c_void);
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(2, batch::UV_SIZE as GLint, gl::FLOAT, 0, batch::VERTEX_SIZE_BYTES as GLsizei, batch::UV_OFFSET_BYTES as *const c_void);
            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(3, batch::TEXTURE_SIZE as GLint, gl::FLOAT, 0, batch::VERTEX_SIZE_BYTES as GLsizei, batch::TEXTURE_OFFSET_BYTES as *const c_void);
            gl::EnableVertexAttribArray(3);


            gl::DrawElements(gl::TRIANGLES, amount as GLsizei, gl::UNSIGNED_INT, null());

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }
}

