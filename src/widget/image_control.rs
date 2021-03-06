use crate::{
    buffer::Buffer,
    widget::traits::ImageTrait,
    Cmd,
    LayoutTree,
    Widget,
};
use image::{
    self,
    DynamicImage,
};
use std::{
    any::Any,
    fmt,
    marker::PhantomData,
};
use stretch::style::Style;

/// Image widget, supported formats: jpg, png
pub struct Image<MSG> {
    image: DynamicImage,
    /// the width of cells used for this image
    width: Option<f32>,
    /// the height of unit cells, will be divided by 2 when used for computing
    /// style layout
    height: Option<f32>,
    id: Option<String>,
    _phantom_msg: PhantomData<MSG>,
}

impl<MSG> Image<MSG> {
    /// create a new image widget
    pub fn new(bytes: Vec<u8>) -> Self {
        let image = Image {
            image: image::load_from_memory(&bytes)
                .expect("unable to load from memory"),
            width: None,
            height: None,
            id: None,
            _phantom_msg: PhantomData,
        };
        image
    }
}

impl<MSG> ImageTrait for Image<MSG> {
    fn width(&self) -> Option<f32> {
        self.width
    }

    fn height(&self) -> Option<f32> {
        self.height
    }

    fn image(&self) -> &DynamicImage {
        &self.image
    }
}

impl<MSG> Widget<MSG> for Image<MSG>
where
    MSG: 'static,
{
    fn style(&self) -> Style {
        self.image_style()
    }

    /// draw this button to the buffer, with the given computed layout
    fn draw(&mut self, buf: &mut Buffer, layout_tree: &LayoutTree) -> Vec<Cmd> {
        self.draw_image(buf, layout_tree)
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

impl<MSG> fmt::Debug for Image<MSG> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Image")
    }
}
