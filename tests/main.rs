use std::hash::{DefaultHasher, Hash, Hasher};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use log::LevelFilter;
use mvutils::once::CreateOnce;
use mvengine::window::app::WindowCallbacks;
use mvengine::window::{UninitializedWindow, Window, WindowCreateInfo};
use mvengine::color::RgbColor;
use mvengine::input::consts::Key;
use mvengine::input::registry::RawInput;
use mvengine::rendering::camera::OrthographicCamera;
use mvengine::rendering::control::RenderController;
use mvengine::rendering::{OpenGLRenderer, Triangle, Vertex};
use mvengine::rendering::shader::OpenGLShader;

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
    renderer: CreateOnce<OpenGLRenderer>,
    camera: CreateOnce<OrthographicCamera>,
    controller: CreateOnce<RenderController>,
}

impl WindowCallbacks for Application {
    fn new(window: UninitializedWindow) -> Self {
        Self {
            renderer: CreateOnce::new(),
            camera: CreateOnce::new(),
            controller: CreateOnce::new(),
        }
    }

    fn post_init(&mut self, window: &mut Window) {
        unsafe {
            let renderer = OpenGLRenderer::initialize(window);
            let camera = OrthographicCamera::new(window.info().width, window.info().height);
            let mut shader = OpenGLShader::new(
                include_str!("index.vert"),
                include_str!("index.frag")
            );
            shader.make().unwrap();
            shader.bind().unwrap();
            shader.use_program();
            let controller = RenderController::new(shader);

            self.renderer.create(|| renderer);
            self.camera.create(|| camera);
            self.controller.create(|| controller);
        }

        let registry = window.input.action_registry_mut();
        registry.create_action("forward");
        registry.create_action("left");
        registry.bind_action("forward", vec![RawInput::KeyPress(Key::W)]);
        registry.bind_action("left", vec![RawInput::KeyPress(Key::A)]);
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

        self.controller.push_triangle(Triangle {
            points: [
                Vertex {
                    pos: (100, 100, 60),
                    color: RgbColor::red(),
                    uv: (0.0, 0.0),
                    texture: 0,
                    has_texture: false,
                },
                Vertex {
                    pos: (100, 200, 60),
                    color: RgbColor::red(),
                    uv: (0.0, 0.0),
                    texture: 0,
                    has_texture: false,
                },
                Vertex {
                    pos: (200, 200, 60),
                    color: RgbColor::red(),
                    uv: (0.0, 0.0),
                    texture: 0,
                    has_texture: false,
                }
            ],
        });

        self.controller.draw(window, &self.camera, &mut *self.renderer);
    }

    fn exiting(&mut self, window: &mut Window) {

    }

    fn resize(&mut self, window: &mut Window, width: u32, height: u32) {

    }
}