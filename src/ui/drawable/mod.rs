pub mod color;
pub mod positioning;
pub mod collection;

use hashbrown::HashMap;
use crate::ui::drawable::collection::LayerDrawable;
use crate::ui::drawable::color::{ColorDrawable};
use crate::ui::drawable::positioning::{PaddedDrawable, RotateDrawable, TranslateDrawable};
use crate::ui::elements::UiElementState;
use crate::ui::styles::Origin;

#[derive(Clone)]
pub struct UiDrawableTransformations {
    translation: (i32, i32),
    size: DrawableSize,
    origin: Origin,
    rotation: f32,
    shrink: (i32, i32)
}

impl UiDrawableTransformations {
    pub(crate) fn modify<F>(&self, mut f: F) -> UiDrawableTransformations where F: FnMut(&mut UiDrawableTransformations) {
        let mut cloned = self.clone();
        f(&mut cloned);
        cloned
    }
}

impl Default for UiDrawableTransformations {
    fn default() -> Self {
        Self {
            translation: (0, 0),
            size: DrawableSize::Scale((1.0, 1.0)),
            origin: Origin::Center,
            rotation: 0.0,
            shrink: (0, 0),
        }
    }
}

#[derive(Clone)]
enum DrawableSize {
    Fixed((i32, i32)),
    Scale((f32, f32))
}

pub enum UiDrawable {
    Color(ColorDrawable),
    Padded(PaddedDrawable),
    Rotate(RotateDrawable),
    Translate(TranslateDrawable),
    Layer(LayerDrawable)
}

pub trait DrawableCallbacks {
    fn draw(&mut self, computed: &UiElementState, transformations: UiDrawableTransformations);
}

impl DrawableCallbacks for UiDrawable {
    fn draw(&mut self, computed: &UiElementState, transformations: UiDrawableTransformations) {
        match self {
            UiDrawable::Color(d) => d.draw(computed, transformations),
            UiDrawable::Padded(d) => d.draw(computed, transformations),
            UiDrawable::Rotate(d) => d.draw(computed, transformations),
            UiDrawable::Translate(d) => d.draw(computed, transformations),
            UiDrawable::Layer(d) => d.draw(computed, transformations),
        }
    }
}

pub trait DrawableCreate {
    fn create(inner: Vec<UiDrawable>, attributes: HashMap<String, String>) -> Result<UiDrawable, String>;
}