use std::mem::offset_of;
use std::os::raw::c_void;
use std::ptr::null;
use gl::types::{GLsizei, GLsizeiptr, GLuint};
use crate::math::vec::{Vec2, Vec3, Vec4};
use crate::rendering::camera::OrthographicCamera;
use crate::rendering::{batch, PrimitiveRenderer, Vertex};
use crate::rendering::shader::OpenGLShader;
use crate::window::Window;

#[repr(C)]
#[derive(Clone)]
pub struct Light {
    pub pos: Vec2,
    pub color: Vec4,
    pub intensity: f32,
    pub range: f32,   // Maximum range of the light
    pub falloff: f32, // How sharply the intensity decays
}

pub struct LightOpenGLRenderer {
    lights: Vec<Light>
}

impl LightOpenGLRenderer {
    pub unsafe fn initialize(window: &Window) -> Self {
        let handle = window.get_handle();

        handle.make_current().expect("Cannot make OpenGL context current");

        Self { lights: vec![] }
    }

    pub fn push_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn lights(&self) -> &Vec<Light> {
        &self.lights
    }

    pub fn lights_mut(&mut self) -> &mut Vec<Light> {
        &mut self.lights
    }
}

impl PrimitiveRenderer for LightOpenGLRenderer {
    fn draw_data(&mut self, window: &Window, camera: &OrthographicCamera, vertices: &[u8], indices: &[u32], textures: &[f32], vbo: GLuint, ibo: GLuint, amount: u32, shader: &mut OpenGLShader) {
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

            shader.uniform_1i("NUM_LIGHTS", self.lights.len() as i32);

            for (i, light) in self.lights.iter().enumerate() {
                let index = i as i32;
                let light_name = format!("LIGHTS[{}]", index);

                shader.uniform_2fv(&format!("{}.pos", light_name), &light.pos);
                shader.uniform_4fv(&format!("{}.color", light_name), &light.color);
                shader.uniform_1f(&format!("{}.intensity", light_name), light.intensity);
                shader.uniform_1f(&format!("{}.range", light_name), light.range);
                shader.uniform_1f(&format!("{}.falloff", light_name), light.falloff);
            }


            let stride = batch::VERTEX_SIZE_BYTES as GLsizei;

            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, stride, offset_of!(Vertex, transform.translation) as *const c_void);
            gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, offset_of!(Vertex, transform.origin) as *const c_void);
            gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, stride, offset_of!(Vertex, transform.scale) as *const c_void);
            gl::VertexAttribPointer(3, 1, gl::FLOAT, gl::FALSE, stride, offset_of!(Vertex, transform.rotation) as *const c_void);

            gl::VertexAttribPointer(4, 3, gl::FLOAT, gl::FALSE, stride, offset_of!(Vertex, pos) as *const c_void);
            gl::VertexAttribPointer(5, 4, gl::FLOAT, gl::FALSE, stride, offset_of!(Vertex, color) as *const c_void);
            gl::VertexAttribPointer(6, 2, gl::FLOAT, gl::FALSE, stride, offset_of!(Vertex, uv) as *const c_void);
            gl::VertexAttribPointer(7, 1, gl::FLOAT, gl::FALSE, stride, offset_of!(Vertex, texture) as *const c_void);
            gl::VertexAttribPointer(8, 1, gl::FLOAT, gl::FALSE, stride, offset_of!(Vertex, has_texture) as *const c_void);

            gl::EnableVertexAttribArray(0);
            gl::EnableVertexAttribArray(1);
            gl::EnableVertexAttribArray(2);
            gl::EnableVertexAttribArray(3);

            gl::EnableVertexAttribArray(4);
            gl::EnableVertexAttribArray(5);
            gl::EnableVertexAttribArray(6);
            gl::EnableVertexAttribArray(7);
            gl::EnableVertexAttribArray(8);

            gl::DrawElements(gl::TRIANGLES, amount as GLsizei, gl::UNSIGNED_INT, null());

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }
}

