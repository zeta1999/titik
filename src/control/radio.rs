use super::{
    Control,
    LayoutTree,
};
use crate::{
    buffer::{
        Buffer,
        Cell,
    },
    symbol,
    symbol::{
        bar,
        line,
        rounded,
    },
};
use stretch::{
    geometry::Size,
    node::{
        Node,
        Stretch,
    },
    number::Number,
    result::Layout,
    style::{
        Dimension,
        Style,
    },
};

#[derive(Default)]
pub struct Radio {
    pub label: String,
    pub is_checked: bool,
}

impl Radio {
    pub fn new<S>(label: S) -> Self
    where
        S: ToString,
    {
        Radio {
            label: label.to_string(),
            ..Default::default()
        }
    }

    pub fn set_label<S: ToString>(&mut self, label: S) {
        self.label = label.to_string();
    }

    pub fn style(&self) -> Style {
        Style {
            size: Size {
                width: Dimension::Points((self.label.len() + 3) as f32),
                height: Dimension::Points(1.0),
            },
            ..Default::default()
        }
    }

    pub fn set_checked(&mut self, checked: bool) {
        self.is_checked = checked;
    }

    /// draw this button to the buffer, with the given computed layout
    pub fn draw(&self, buf: &mut Buffer, layout_tree: &LayoutTree) {
        let layout = layout_tree.layout;
        let loc_x = layout.location.x.round() as usize;
        let loc_y = layout.location.y.round() as usize;
        let (box_symbol, x_offset) = if self.is_checked {
            (symbol::RADIO_CHECKED, 1)
        } else {
            (symbol::RADIO_UNCHECKED, 0)
        };
        buf.set_symbol(loc_x + 1, loc_y + 1, box_symbol);

        for (t, ch) in self.label.chars().enumerate() {
            buf.set_symbol(loc_x + 3 + x_offset + t, loc_y + 1, ch);
        }
    }
}

impl From<Radio> for Control {
    fn from(btn: Radio) -> Self {
        Control::Radio(btn)
    }
}
