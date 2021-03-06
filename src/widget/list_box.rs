use crate::{
    buffer::Buffer,
    Cmd,
    LayoutTree,
    Widget,
};
use ito_canvas::unicode_canvas::{
    Border,
    Canvas,
};
use sauron_vdom::Callback;
use std::{
    any::Any,
    fmt,
};
use stretch::{
    geometry::Size,
    style::{
        Dimension,
        FlexDirection,
        Style,
    },
};

/// a flex box
#[derive(Default, Debug)]
pub struct ListBox<MSG> {
    list: Vec<String>,
    width: Option<f32>,
    height: Option<f32>,
    flex_direction: FlexDirection,
    scroll_top: f32,
    on_input: Vec<Callback<sauron_vdom::Event, MSG>>,
    id: Option<String>,
    use_divider: bool,
}

impl<MSG> ListBox<MSG> {
    ///create a new flexbox
    pub fn new() -> Self {
        ListBox {
            width: None,
            height: None,
            flex_direction: FlexDirection::Row,
            scroll_top: 0.0,
            on_input: vec![],
            list: vec![],
            id: None,
            use_divider: true,
        }
    }

    fn draw_border(&mut self, buf: &mut Buffer, layout_tree: &LayoutTree) {
        let layout = layout_tree.layout;
        let loc_x = layout.location.x.round() as usize;
        let loc_y = layout.location.y.round() as usize;
        let width = layout.size.width.round() as usize;
        let height = layout.size.height.round() as usize;

        let left = loc_x;
        let top = loc_y;
        let bottom = top + height - 1;
        let right = left + width - 1;

        let border = Border {
            use_thick_border: false,
            has_top: true,
            has_bottom: true,
            has_left: true,
            has_right: true,
            is_top_left_rounded: false,
            is_top_right_rounded: false,
            is_bottom_left_rounded: false,
            is_bottom_right_rounded: false,
        };
        let mut canvas = Canvas::new();
        canvas.draw_rect((left, top), (right, bottom), border);
        buf.write_canvas(canvas);
    }

    /// set the list of this listbox;
    pub fn set_list(&mut self, list: Vec<String>) {
        self.list = list;
    }

    fn draw_items(&self, buf: &mut Buffer, layout_tree: &LayoutTree) {
        let layout = layout_tree.layout;
        let loc_x = layout.location.x.round() as usize;
        let loc_y = layout.location.y.round() as usize;
        let width = layout.size.width.round() as usize;
        for (j, li) in self.list.iter().enumerate() {
            if self.use_divider {
                let mut canvas = Canvas::new();
                buf.write_str(loc_x + 2, loc_y + 1 + (j * 2), li);
                let left = loc_x + 1;
                let right = loc_x + width - 2;
                let bottom = loc_y + 1 + (j * 2) + 1;
                canvas.draw_horizontal_line(
                    (left, bottom),
                    (right, bottom),
                    false,
                );
                buf.write_canvas(canvas);
            }
        }
    }
}

impl<MSG> Widget<MSG> for ListBox<MSG>
where
    MSG: fmt::Debug + 'static,
{
    fn style(&self) -> Style {
        Style {
            size: Size {
                width: if let Some(width) = self.width {
                    Dimension::Points(width)
                } else {
                    Dimension::Percent(1.0)
                },
                height: if let Some(height) = self.height {
                    Dimension::Points(height)
                } else {
                    Dimension::Percent(1.0)
                },
            },
            min_size: Size {
                width: if let Some(width) = self.width {
                    Dimension::Points(width)
                } else {
                    Dimension::Auto
                },
                height: if let Some(height) = self.height {
                    Dimension::Points(height)
                } else {
                    Dimension::Points(1.0)
                },
            },
            ..Default::default()
        }
    }

    fn draw(&mut self, buf: &mut Buffer, layout_tree: &LayoutTree) -> Vec<Cmd> {
        self.draw_border(buf, layout_tree);
        self.draw_items(buf, layout_tree);
        vec![]
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn set_size(&mut self, width: Option<f32>, height: Option<f32>) {
        self.width = width;
        self.height = height;
    }

    fn set_id(&mut self, id: &str) {
        self.id = Some(id.to_string());
    }

    fn get_id(&self) -> &Option<String> {
        &self.id
    }
}
