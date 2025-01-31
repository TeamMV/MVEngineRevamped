use crate::input::{Input, RawInputEvent};
use crate::input::registry::ActionInputProcessor;

pub trait InputProcessor {
    fn digest_action(&mut self, action: RawInputEvent, input: &Input);
    fn end_frame(&mut self);
    fn set_enabled(&mut self, state: bool);
    fn is_enabled(&self) -> bool;

    fn enable(&mut self) {
        self.set_enabled(true);
    }

    fn disable(&mut self) {
        self.set_enabled(false);
    }
}

pub struct InputCollector {
    pub(crate) action_processor: ActionInputProcessor,
    targets: Vec<Box<dyn InputProcessor>>
}

impl InputCollector {
    pub fn new() -> Self {
        Self {
            action_processor: ActionInputProcessor::new(),
            targets: vec![],
        }
    }

    pub fn dispatch_input(&mut self, action: RawInputEvent, input: &Input) {
        self.action_processor.digest_action(action, input);
        for target in &mut self.targets {
            if target.is_enabled() {
                target.digest_action(action, input);
            }
        }
    }

    pub fn end_frame(&mut self) {
        self.action_processor.end_frame();
        for target in &mut self.targets {
            if target.is_enabled() {
                target.end_frame();
            }
        }
    }
}