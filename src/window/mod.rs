pub mod app;

use std::mem;
use crate::window::app::WindowCallbacks;
use std::time::SystemTime;
use glutin::{ElementState, Event, MouseButton, MouseScrollDelta, VirtualKeyCode, WindowBuilder};
use hashbrown::HashSet;
use mvutils::once::CreateOnce;
use mvutils::unsafe_utils::{DangerousCell, Unsafe};
use crate::input::collect::InputCollector;
use crate::input::{Input, KeyboardAction, MouseAction, RawInputEvent};
use crate::input::consts::{Key};

const NANOS_PER_SEC: u64 = 1_000_000_000;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct WindowCreateInfo {
    /// The width of the window in pixels.
    ///
    /// Default is 800.
    pub width: u32,

    /// The height of the window in pixels.
    ///
    /// Default is 600.
    pub height: u32,

    /// The window title, which is displayed at the top of the window.
    ///
    /// Default is an empty string.
    pub title: String,

    /// Whether the window should be fullscreen.
    ///
    /// Default is false.
    pub fullscreen: bool,

    /// Whether the window should have a frame and buttons (like close, minimize and maximize)
    ///
    /// Default is true.
    pub decorated: bool,

    /// Whether the window should be resizable.
    ///
    /// Default is true.
    pub resizable: bool,

    /// Whether the window background is transparent.
    ///
    /// Default is false.
    pub transparent: bool,

    /// Dark or Light theme. None means system theme.
    ///
    /// Default is None.
    //pub theme: Option<Theme>,

    /// Whether to sync the screen update with the time the vertical electron beam of your monitor reaches its lowest point.
    ///
    /// Default is true.
    pub vsync: bool,

    /// The maximum frames in flight of the rendering API.
    ///
    /// Default is 2.
    pub max_frames_in_flight: u32,

    /// The maximum frames per second of the window.
    ///
    /// Default is 60.
    pub fps: u32,

    /// The maximum updates per second of the window.
    ///
    /// Default is 20.
    pub ups: u32,
}

impl Default for WindowCreateInfo {
    fn default() -> Self {
        WindowCreateInfo {
            width: 800,
            height: 600,
            title: "MVEngine Application".to_string(),
            fullscreen: false,
            decorated: true,
            resizable: true,
            transparent: false,
            //theme: None,
            vsync: false,
            max_frames_in_flight: 2,
            fps: 60,
            ups: 20,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum State {
    Ready,
    Running,
    Exited,
}

pub struct Window {
    pub(crate) info: WindowCreateInfo,

    handle: CreateOnce<glutin::Window>,
    state: State,

    frame_time_nanos: u64,
    update_time_nanos: u64,
    delta_t: f64,
    delta_u: f64,
    time_f: SystemTime,
    time_u: SystemTime,

    pub input: Input,
    pressed_keys: HashSet<Key>
}

impl Window {
    pub fn new(info: WindowCreateInfo) -> Self {
        let frame_time_nanos = NANOS_PER_SEC / info.fps as u64;
        let update_time_nanos = NANOS_PER_SEC / info.ups as u64;

        Window {
            info,

            handle: CreateOnce::new(),
            state: State::Ready,

            frame_time_nanos,
            update_time_nanos,
            delta_t: 0.0,
            delta_u: 0.0,
            time_f: SystemTime::now(),
            time_u: SystemTime::now(),
            input: Input::new(),
            pressed_keys: HashSet::new(),
        }
    }

    pub fn run<T: WindowCallbacks + 'static>(mut self) -> Result<(), ()> {
        let mut app_loop = T::new(UninitializedWindow { inner: &mut self });

        let mut window = WindowBuilder::new()
            .with_visibility(false)
            .with_dimensions(self.info.width, self.info.height)
            .with_title(self.info.title.clone())
            .with_decorations(self.info.decorated);
        if self.info.vsync {
            window = window.with_vsync();
        }

        let w = window.build().map_err(|_| ())?;
        unsafe { w.make_current().map_err(|_| ())?; }
        gl::load_with(|symbol| {
            w.get_proc_address(symbol) as *const _
        });

        self.handle.create(|| w);

        let this = unsafe { Unsafe::cast_mut_static(&mut self) };
        let this2 = unsafe { Unsafe::cast_mut_static(&mut self) };
        app_loop.post_init(&mut self);

        self.handle.show();
        self.state = State::Running;

        'outer: loop {
            for event in this2.handle.poll_events() {
                match event {
                    Event::Resized(w, h) => {
                        self.info.width = w;
                        self.info.height = h;
                        app_loop.resize(&mut self, w, h);
                    }
                    Event::Moved(_, _) => {}
                    Event::Closed => {
                        break 'outer;
                    }
                    Event::DroppedFile(_) => {}
                    Event::ReceivedCharacter(_) => {}
                    Event::Focused(_) => {}
                    Event::KeyboardInput(state, code, Some(key)) => {
                        let code = unsafe { mem::transmute::<VirtualKeyCode, Key>(key) };
                        let event = match state {
                            ElementState::Pressed => {
                                if !self.pressed_keys.contains(&code) {
                                    self.pressed_keys.insert(code.clone());
                                    RawInputEvent::Keyboard(KeyboardAction::Press(code))
                                } else {
                                    RawInputEvent::Keyboard(KeyboardAction::Type(code))
                                }
                            },
                            ElementState::Released => {
                                self.pressed_keys.remove(&code);
                                RawInputEvent::Keyboard(KeyboardAction::Release(code))
                            }
                        };
                        this.input.collector.dispatch_input(event, &self.input);
                    }
                    Event::MouseMoved(x, y) => {
                        this.input.collector.dispatch_input(RawInputEvent::Mouse(MouseAction::Move(x, y)), &self.input);
                        self.input.mouse_x = x;
                        self.input.mouse_y = y;
                    }
                    Event::MouseWheel(delta, touch_phase, x) => {
                        if let MouseScrollDelta::PixelDelta(dx, dy) = delta {
                            this.input.collector.dispatch_input(RawInputEvent::Mouse(MouseAction::Wheel(dx, dy)), &self.input);
                        }
                    }
                    Event::MouseInput(i, d, k) => {
                        let button = unsafe { mem::transmute::<MouseButton, crate::input::consts::MouseButton>(d) };
                        match i {
                            ElementState::Pressed => {
                                this.input.collector.dispatch_input(RawInputEvent::Mouse(MouseAction::Press(button)), &self.input);
                            }
                            ElementState::Released => {
                                this.input.collector.dispatch_input(RawInputEvent::Mouse(MouseAction::Release(button)), &self.input);
                            }
                        }
                    }
                    Event::TouchpadPressure(_, _) => {}
                    Event::Awakened => {}
                    Event::Refresh => {}
                    Event::Suspended(_) => {}
                    Event::Touch(_) => {}
                    _ => {}
                }
            }

            let elapsed = self.time_u.elapsed().expect("SystemTime error").as_nanos();
            if elapsed > self.update_time_nanos as u128 {
                self.time_u = SystemTime::now();
                self.delta_u = elapsed as f64 / NANOS_PER_SEC as f64;
                let delta_u = self.delta_u;

                app_loop.update(&mut self, delta_u);
            }

            let elapsed = self.time_f.elapsed().expect("SystemTime error").as_nanos();
            if elapsed > self.frame_time_nanos as u128 {
                self.time_f = SystemTime::now();
                self.delta_t = elapsed as f64 / NANOS_PER_SEC as f64;
                let delta_t = self.delta_t;

                app_loop.draw(&mut self, delta_t);
                self.input.collector.end_frame();
                self.handle.swap_buffers().map_err(|_| ())?;
            }
        }

        self.state = State::Exited;
        app_loop.exiting(&mut self);

        Ok(())
    }

    pub fn info(&self) -> &WindowCreateInfo {
        &self.info
    }

    pub fn get_state(&self) -> State {
        self.state
    }

    pub fn get_handle(&self) -> &glutin::Window {
        &self.handle
    }

    pub fn get_delta_t(&self) -> f64 {
        self.delta_t
    }

    pub fn get_delta_u(&self) -> f64 {
        self.delta_u
    }

    pub fn set_fps(&mut self, fps: u32) {
        self.info.fps = fps;
        self.frame_time_nanos = NANOS_PER_SEC / fps as u64;
    }

    pub fn set_ups(&mut self, ups: u32) {
        self.info.ups = ups;
        self.update_time_nanos = NANOS_PER_SEC / ups as u64;
    }

    pub fn fps(&self) -> u32 {
        (1.0 / self.delta_t) as u32
    }

    pub fn ups(&self) -> u32 {
        (1.0 / self.delta_u) as u32
    }

    pub fn center(&self) -> (i32, i32) {
        ((self.info.width / 2) as i32, (self.info.height / 2) as i32)
    }
}

pub struct UninitializedWindow<'window> {
    inner: &'window mut Window
}

impl<'window> UninitializedWindow<'window> {
    pub fn info(&self) -> &WindowCreateInfo {
        &self.inner.info()
    }

    pub fn get_state(&self) -> State {
        self.inner.get_state()
    }

    pub fn get_delta_t(&self) -> f64 {
        self.inner.get_delta_t()
    }

    pub fn get_delta_u(&self) -> f64 {
        self.inner.get_delta_u()
    }

    pub fn set_fps(&mut self, fps: u32) {
        self.inner.set_fps(fps)
    }

    pub fn set_ups(&mut self, ups: u32) {
        self.inner.set_ups(ups)
    }

    pub fn fps(&self) -> u32 {
        self.inner.fps()
    }

    pub fn ups(&self) -> u32 {
        self.inner.ups()
    }

    pub fn center(&self) -> (i32, i32) {
        self.inner.center()
    }
}