use log::LevelFilter;
use mvengine::color::RgbColor;
use mvengine::input::consts::Key;
use mvengine::input::registry::RawInput;
use mvengine::math::vec::Vec2;
use mvengine::rendering::camera::OrthographicCamera;
use mvengine::rendering::control::RenderController;
use mvengine::rendering::light::{Light, LightOpenGLRenderer};
use mvengine::rendering::shader::light::LightOpenGLShader;
use mvengine::rendering::{Transform, Triangle, Vertex};
use mvengine::window::app::WindowCallbacks;
use mvengine::window::{UninitializedWindow, Window, WindowCreateInfo};
use mvutils::once::CreateOnce;
use std::hash::Hash;
use gl::types::GLint;
use mvengine::rendering::post::{OpenGLPostProcessRenderer, OpenGLPostProcessShader};

pub fn main() -> Result<(), ()> {
    mvlogger::init(std::io::stdout(), LevelFilter::Trace);
    let mut info = WindowCreateInfo::default();
    info.title = "Window demo".to_string();
    info.fps = 60;
    info.ups = 20;
    info.vsync = true;

    let window = Window::new(info);
    window.run::<Application>()
}

struct Application {
    renderer: CreateOnce<LightOpenGLRenderer>,
    camera: CreateOnce<OrthographicCamera>,
    controller: CreateOnce<RenderController>,
    shader: CreateOnce<LightOpenGLShader>,
    post_render: CreateOnce<OpenGLPostProcessRenderer>,
    invert_shader: CreateOnce<OpenGLPostProcessShader>
}

impl WindowCallbacks for Application {
    fn new(window: UninitializedWindow) -> Self {
        Self {
            renderer: CreateOnce::new(),
            camera: CreateOnce::new(),
            controller: CreateOnce::new(),
            shader: CreateOnce::new(),
            post_render: CreateOnce::new(),
            invert_shader: CreateOnce::new()
        }
    }

    fn post_init(&mut self, window: &mut Window) {
        unsafe {
            let mut renderer = LightOpenGLRenderer::initialize(window);
            renderer.push_light(Light {
                pos: Vec2::new(250.0, 175.0),
                color: RgbColor::yellow().as_vec4(),
                intensity: 200.0,
                range: 200.0,
                falloff: 0.2,
            });

            renderer.push_light(Light {
                pos: Vec2::new(550.0, 175.0),
                color: RgbColor::green().as_vec4(),
                intensity: 20000.0,
                range: 500.0,
                falloff: 3.0,
            });

            let camera = OrthographicCamera::new(window.info().width, window.info().height);
            let mut shader = LightOpenGLShader::new();
            shader.make().unwrap();
            shader.bind().unwrap();
            shader.use_program();
            let controller = RenderController::new(shader.get_program_id());

            let post_render = OpenGLPostProcessRenderer::new(window.info().width as i32, window.info().height as i32);

            let mut post_shader = OpenGLPostProcessShader::new(include_str!("invert.frag"));
            post_shader.make().unwrap();
            post_shader.bind().unwrap();


            self.renderer.create(|| renderer);
            self.camera.create(|| camera);
            self.controller.create(|| controller);
            self.shader.create(|| shader);
            self.post_render.create(|| post_render);
            self.invert_shader.create(|| post_shader);
        }

        let registry = window.input.action_registry_mut();
        registry.create_action("forward");
        registry.create_action("left");
        registry.bind_action("forward", vec![RawInput::KeyPress(Key::W)]);
        registry.bind_action("left", vec![RawInput::KeyPress(Key::A)]);

        registry.create_action("save");
        registry.bind_action("save", vec![RawInput::KeyPress(Key::LControl), RawInput::KeyPress(Key::S)]);
    }

    fn update(&mut self, window: &mut Window, delta_u: f64) {

    }

    fn draw(&mut self, window: &mut Window, delta_t: f64) {
        if window.input.is_action("forward") {
            println!("forward is triggered");
        }

        if window.input.is_action("left") {
            println!("left is triggered");
        }

        if window.input.was_action("save") {
            println!("save was triggered");
        }

        let trns = Transform {
            translation: Default::default(),
            origin: Vec2::new(150.0, 150.0),
            scale: Vec2::splat(1.0),
            rotation: 0f32.to_radians(),
        };

        self.controller.push_triangle(Triangle {
            points: [
                Vertex {
                    transform: trns.clone(),
                    pos: (100.0, 100.0, 60.0),
                    color: RgbColor::white().as_vec4(),
                    uv: (0.0, 0.0),
                    texture: 0.0,
                    has_texture: 0.0,
                },
                Vertex {
                    transform: trns.clone(),
                    pos: (300.0, 400.0, 60.0),
                    color: RgbColor::white().as_vec4(),
                    uv: (0.0, 0.0),
                    texture: 0.0,
                    has_texture: 0.0,
                },
                Vertex {
                    transform: trns,
                    pos: (500.0, 100.0, 60.0),
                    color: RgbColor::white().as_vec4(),
                    uv: (0.0, 0.0),
                    texture: 0.0,
                    has_texture: 0.0,
                }
            ],
        });

        let target = self.controller.draw_to_target(window, &self.camera, &mut *self.renderer, &mut *self.shader);
        //idk second shader only receives black texture

        self.post_render.set_target(target);
        self.post_render.run_shader(&mut *self.invert_shader);
        self.post_render.draw_to_screen();
    }

    fn exiting(&mut self, window: &mut Window) {

    }

    fn resize(&mut self, window: &mut Window, width: u32, height: u32) {

    }
}