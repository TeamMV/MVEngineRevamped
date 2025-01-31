use hashbrown::HashMap;
use log::error;
use crate::ui::drawable::{DrawableCallbacks, DrawableCreate, UiDrawable, UiDrawableTransformations};
use crate::ui::styles::{Dimension, Location};
use mvcore::color::{ColorFormat, RgbColor};
use mvutils::utils::{Map, PClamp, Percentage};
use num_traits::AsPrimitive;
use crate::ui::elements::UiElementState;

pub struct ColorDrawable {
    pub color: RgbColor,
}

impl ColorDrawable {
    pub fn new(color: RgbColor) -> Self {
        Self { color }
    }
}

impl DrawableCallbacks for ColorDrawable {
    fn draw(&mut self, computed: &UiElementState, transformations: UiDrawableTransformations) {
        let origin = &computed.transforms.origin;
        let x = computed.rect.x();
        let y = computed.rect.y();

        let width = computed.rect.width();
        let height = computed.rect.height();

        let ox = origin.get_actual_x(x, width, computed);
        let oy = origin.get_actual_y(y, height, computed);

        let rotation = computed.transforms.rotation + transformations.rotation;

        let x = computed.rect.x() + computed.transforms.translation.width;
        let y = computed.rect.y() + computed.transforms.translation.height;
    }
}

impl DrawableCreate for ColorDrawable {
    fn create(inner: Vec<UiDrawable>, attributes: HashMap<String, String>) -> Result<UiDrawable, String> {
        if !inner.is_empty() {
            error!("ColorDrawable cannot have inner Drawables!");
        }
        let col_str = attributes.get("color").expect("Expected 'color' attribute on ColorDrawable");
        let color = mvcore::color::parse::parse_color(col_str);
        if color.is_err() {
            return Err(color.unwrap_err().1);
        }
        let color = color.unwrap();
        Ok(UiDrawable::Color(ColorDrawable::new(color)))
    }
}