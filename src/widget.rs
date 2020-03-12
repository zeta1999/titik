use crate::{
    buffer::{
        Buffer,
        Cell,
    },
    symbol::{
        bar,
        line,
        rounded,
    },
};
pub use button::Button;
pub use checkbox::Checkbox;
use crossterm::style::Color;
pub use flex_box::FlexBox;
pub use image_control::Image;
pub use layout::{
    compute_layout,
    LayoutTree,
};
pub use radio::Radio;
use std::boxed;
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
pub use text_input::TextInput;

mod button;
mod checkbox;
mod flex_box;
mod image_control;
mod layout;
mod radio;
mod text_input;

pub trait Widget {
    fn style(&self) -> Style;
    fn add_child(&mut self, child: boxed::Box<Widget>) -> bool {
        false
    }

    fn children(&self) -> Option<&[boxed::Box<dyn Widget>]> {
        None
    }
    fn draw(&self, but: &mut Buffer, layout_tree: &LayoutTree);

    fn style_node(&self, stretch: &mut Stretch) -> Option<Node> {
        let children_styles = if let Some(children) = self.children() {
            children
                .iter()
                .filter_map(|c| c.style_node(stretch))
                .collect()
        } else {
            vec![]
        };
        stretch.new_node(self.style(), children_styles).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::boxed;
    use stretch::{
        geometry::*,
        result::*,
        style::*,
    };

    #[test]
    fn layout() {
        let mut control = FlexBox::new();
        control.horizontal();
        let mut btn = Button::new("Hello");
        btn.set_size(Some(30.0), Some(34.0));

        control.add_child(boxed::Box::new(btn));

        let mut btn = Button::new("world");
        btn.set_size(Some(20.0), Some(10.0));
        control.add_child(boxed::Box::new(btn));

        let layout_tree = compute_layout(
            &mut control,
            Size {
                width: Number::Defined(100.0),
                height: Number::Defined(100.0),
            },
        );

        let layout1 = layout_tree.children_layout[1].layout;
        assert_eq!(
            layout1.size,
            Size {
                width: 20.0,
                height: 10.0
            }
        );

        assert_eq!(layout1.location, Point { x: 30.0, y: 0.0 });
    }

    #[test]
    fn layout2() {
        let mut control = FlexBox::new();
        control.vertical();

        let mut btn1 = Button::new("Hello");
        btn1.set_size(Some(100.0), Some(20.0));

        control.add_child(boxed::Box::new(btn1));

        let mut btn2 = Button::new("world");
        btn2.set_size(Some(20.0), Some(10.0));

        let mut btn3 = Button::new("world");
        btn3.set_size(Some(20.0), Some(10.0));

        let mut hrow = FlexBox::new();
        hrow.horizontal();

        hrow.add_child(boxed::Box::new(btn2));
        hrow.add_child(boxed::Box::new(btn3));

        control.add_child(boxed::Box::new(hrow));

        let layout_tree = compute_layout(
            &mut control,
            Size {
                width: Number::Defined(100.0),
                height: Number::Defined(100.0),
            },
        );

        println!("{:#?}", layout_tree);

        let layout_btn2 =
            layout_tree.children_layout[1].children_layout[1].layout;
        assert_eq!(layout_btn2.location, Point { x: 20.0, y: 20.0 });
    }
}