use crate::ui::elements::{UiElement, UiElementCallbacks, UiElementStub};
use mvcore::input::raw::Input;
use mvcore::input::{InputAction, InputProcessor, KeyboardAction, MouseAction};
use mvutils::once::{CreateOnce, Lazy};
use mvutils::unsafe_utils::{DangerousCell, Unsafe};
use mvutils::utils::Recover;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use crate::input::InputAction;
use crate::ui::context::{UiContext, UiResources};
use crate::ui::render::ctx::DrawContext2D;

pub mod anim;
pub mod attributes;
pub mod drawable;
pub mod ease;
pub mod elements;
pub mod parse;
pub mod prelude;
pub mod styles;
pub mod timing;
pub mod uix;
pub mod utils;
pub mod theme;
pub mod geometry;
pub mod render;
pub mod res;
pub mod context;

pub(crate) static mut UI: Lazy<Arc<DangerousCell<Ui>>> =
    Lazy::new(|| Arc::new(DangerousCell::new(Ui::new())));

pub fn ui() -> &'static Ui {
    unsafe { UI.get() }
}

pub fn ui_mut() -> &'static mut Ui {
    unsafe { UI.get_mut() }
}

pub struct Ui {
    context: CreateOnce<UiContext>,
    input: CreateOnce<Arc<DangerousCell<Input>>>,
    enabled: bool,
    root_elems: Vec<Rc<DangerousCell<UiElement>>>,
}

impl Ui {
    fn new() -> Self {
        unsafe { if UI.created() {} }

        Self {
            context: CreateOnce::new(),
            input: CreateOnce::new(),
            enabled: true,
            root_elems: vec![],
        }
    }

    pub fn init(&mut self, resources: &'static dyn UiResources) {
        self.context.create(|| UiContext::new(resources));
    }

    pub fn init_input(&mut self, input: Arc<DangerousCell<Input>>) {
        self.input.create(move || input);
    }

    pub fn context(&self) -> UiContext {
        self.context.clone()
    }

    pub fn add_root(&mut self, elem: Rc<DangerousCell<UiElement>>) {
        self.root_elems.push(elem);
    }

    pub fn remove_root(&mut self, elem: Rc<DangerousCell<UiElement>>) {
        self.root_elems.retain(|e| {
            let guard1 = e.get();
            let guard2 = elem.get();
            guard1.attributes().id != guard2.attributes().id
        })
    }

    pub fn compute_styles(&mut self) {
        for arc in self.root_elems.iter_mut() {
            let mut guard = arc.get_mut();
            guard.compute_styles();
        }
    }

    pub fn draw(&mut self, ctx: &mut DrawContext2D) {
        for arc in self.root_elems.iter_mut() {
            let mut guard = arc.get_mut();
            guard.draw(ctx);
        }
    }

    pub fn compute_styles_and_draw(&mut self, ctx: &mut DrawContext2D) {
        for arc in self.root_elems.iter_mut() {
            let mut guard = arc.get_mut();
            guard.compute_styles();
            guard.draw(ctx);
        }
    }

    pub fn input_processor(action: InputAction) {
        match action {
            InputAction::Keyboard(k) => unsafe {
                UI.get_mut().keyboard_change(k);
            },
            InputAction::Mouse(m) => unsafe {
                UI.get_mut().mouse_change(m);
            },
        }
    }
}

impl InputProcessor for Ui {
    fn new(input: Arc<DangerousCell<Input>>) -> Self
    where
        Self: Sized,
    {
        unimplemented!()
    }

    fn input(&self) -> Arc<DangerousCell<Input>> {
        self.input.clone()
    }

    fn mouse_change(&mut self, action: MouseAction) {
        let input = self.input.get_mut();

        unsafe {
            for root in Unsafe::cast_static(&self.root_elems) {
                let mut guard = root.get_mut();
                let mut guard_ref = Unsafe::cast_mut_static(&mut guard);
                let mut events = &mut guard.state_mut().events;
                events.mouse_change(action, &mut *guard_ref, &*input);
            }
        }
    }

    fn keyboard_change(&mut self, action: KeyboardAction) {
        let input = self.input.get_mut();

        unsafe {
            for root in Unsafe::cast_static(&self.root_elems) {
                let mut guard = root.get_mut();
                let mut guard_ref = Unsafe::cast_mut_static(&mut guard);
                let mut events = &mut guard.state_mut().events;
                events.keyboard_change(action, &mut *guard_ref, &*input);
            }
        }
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    fn is_enabled(&self) -> bool {
        self.enabled
    }
}
