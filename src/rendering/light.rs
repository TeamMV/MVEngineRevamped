use crate::math::vec::{Vec2, Vec4};
use crate::rendering::camera::OrthographicCamera;
use crate::rendering::post::RenderTarget;
use crate::rendering::shader::OpenGLShader;
use crate::rendering::{batch, PrimitiveRenderer, Vertex};
use crate::window::Window;
use gl::types::{GLsizei, GLsizeiptr, GLuint};
use std::mem::offset_of;
use std::os::raw::c_void;
use std::ptr::null;

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
    lights: Vec<Light>,
    framebuffer: GLuint,
    offscreen_target: GLuint
}

impl LightOpenGLRenderer {
    pub unsafe fn initialize(window: &Window) -> Self {
        let handle = window.get_handle();

        handle.make_current().expect("Cannot make OpenGL context current");

        let mut offscreen_target = 0;
        gl::GenTextures(1, &mut offscreen_target);
        gl::BindTexture(gl::TEXTURE_2D, offscreen_target);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as i32,  // Ensure correct internal format
            window.info.width as GLsizei,
            window.info.height as GLsizei,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            null(),
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        let mut fb = 0;
        gl::GenFramebuffers(1, &mut fb);
        gl::BindFramebuffer(gl::FRAMEBUFFER, fb);
        gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, offscreen_target, 0);

        let attachments = [gl::COLOR_ATTACHMENT0];
        gl::DrawBuffers(1, attachments.as_ptr());

        //let mut db = 0;
        //gl::GenRenderbuffers(1, &mut db);
        //gl::BindRenderbuffer(gl::RENDERBUFFER, db);
        //gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH_COMPONENT, window.info.width as GLsizei, window.info.height as GLsizei);
        //gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, gl::RENDERBUFFER, db);

        Self {
            lights: vec![],
            framebuffer: fb,
            offscreen_target,
        }
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
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, indices.len() as GLsizeiptr * 4, indices.as_ptr() as *const _, gl::DYNAMIC_DRAW);

            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

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

            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    fn draw_data_to_target(&mut self, window: &Window, camera: &OrthographicCamera, vertices: &[u8], indices: &[u32], textures: &[f32], vbo: GLuint, ibo: GLuint, amount: u32, shader: &mut OpenGLShader, post: &mut RenderTarget) {
        post.framebuffer = self.framebuffer;
        post.texture = self.offscreen_target;
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER, vertices.len() as GLsizeiptr, vertices.as_ptr() as *const _, gl::DYNAMIC_DRAW);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, indices.len() as GLsizeiptr * 4, indices.as_ptr() as *const _, gl::DYNAMIC_DRAW);

            gl::BindFramebuffer(gl::FRAMEBUFFER, self.framebuffer);

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

            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);

            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}

