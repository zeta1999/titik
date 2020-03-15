use crossterm::event::EnableMouseCapture;
pub use crossterm::{
    cursor,
    event::{
        self,
        Event,
        KeyCode,
        KeyEvent,
        KeyModifiers,
        MouseEvent,
    },
    execute,
    queue,
    style,
    style::{
        Attribute,
        Attributes,
        Color,
        ContentStyle,
    },
    terminal::{
        self,
        ClearType,
    },
    Command,
    Result,
};
use std::{
    cell::RefCell,
    io::{
        self,
        Write,
    },
    rc::Rc,
};
use stretch::{
    geometry::{
        Rect,
        Size,
    },
    number::Number,
    style::{
        Dimension,
        FlexDirection,
        Style,
        *,
    },
};
use titik::{
    compute_layout,
    find_widget,
    find_widget_mut,
    set_focused_node,
    widget_hit_at,
    widget_node_idx_at,
    Buffer,
    Button,
    Checkbox,
    Cmd,
    FlexBox,
    Image,
    InputBuffer,
    LayoutTree,
    Radio,
    TextInput,
    Widget,
};

fn init<W: Write>(w: &mut W) -> Result<()> {
    execute!(w, terminal::EnterAlternateScreen)?;
    execute!(w, EnableMouseCapture)?;
    terminal::enable_raw_mode()?;
    Ok(())
}

fn finalize<W: Write>(w: &mut W) -> Result<()> {
    execute!(
        w,
        style::ResetColor,
        cursor::Show,
        terminal::LeaveAlternateScreen
    )?;
    terminal::disable_raw_mode()?;
    w.flush()?;
    Ok(())
}

fn clear<W: Write>(w: &mut W) -> crossterm::Result<()> {
    queue!(
        w,
        style::ResetColor,
        terminal::Clear(ClearType::All),
        cursor::Show,
        cursor::MoveTo(1, 1)
    )?;
    Ok(())
}

fn run<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    init(w)?;

    let mut focused_widget_idx = None;
    let mut events = String::new();
    let mut commands = String::new();

    let mut root_node = FlexBox::new();
    let mut cb1 = Checkbox::new(format!("{:?}", focused_widget_idx));
    cb1.set_checked(true);
    let mut cb2 = Checkbox::new("Checkbox2");
    cb2.set_checked(false);
    let mut rb1 = Radio::new("Radio1");
    rb1.set_checked(true);
    let mut input1 = TextInput::new("Hello world!");

    let mut input2 = TextInput::new("Commands");
    input2.set_value(&commands);

    let rb2 = Radio::new("Radio2");
    let mut btn2 = Button::new(format!("Events: {}", events));
    btn2.set_rounded(true);
    let mut img = Image::new(include_bytes!("../horse.jpg").to_vec());
    img.set_size(Some(80.0), Some(40.0));
    root_node.vertical();

    let btn1 = Button::new("Button 1");
    root_node.add_child(Box::new(btn1));
    root_node.add_child(Box::new(btn2));
    root_node.add_child(Box::new(img));
    root_node.add_child(Box::new(cb2));
    root_node.add_child(Box::new(cb1));

    root_node.add_child(Box::new(rb1));
    root_node.add_child(Box::new(rb2));
    root_node.add_child(Box::new(input1));
    root_node.add_child(Box::new(input2));

    loop {
        let (width, height) = buffer_size().unwrap();
        root_node.set_size(Some((width - 2) as f32), Some(height as f32));
        let layout_tree = compute_layout(
            &mut root_node,
            Size {
                width: Number::Defined(width as f32),
                height: Number::Defined(height as f32),
            },
        );

        //draw before and after the event reader
        draw_buffer(w, &root_node, &layout_tree, width, height);

        if let Ok(ev) = event::read() {
            events = format!("{:?}", ev);
            match ev {
                Event::Key(key_event) => {
                    // To quite, press any of the following:
                    //  - CTRL-c
                    //  - CTRL-q
                    //  - CTRL-d
                    //  - CTRL-z
                    if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                        match key_event.code {
                            KeyCode::Char(c) => {
                                match c {
                                    'c' | 'q' | 'd' | 'z' => {
                                        break;
                                    }
                                    _ => (),
                                }
                            }
                            _ => (),
                        }
                    } else {
                        if let Some(idx) = focused_widget_idx.as_ref() {
                            let active_widget: Option<&mut dyn Widget> =
                                find_widget_mut(&mut root_node, *idx);
                            if let Some(focused_widget) = active_widget {
                                if let Some(txt_input1) = focused_widget
                                    .as_any_mut()
                                    .downcast_mut::<TextInput>()
                                {
                                    txt_input1.process_key(key_event);
                                }
                            }
                        }
                    }
                }
                Event::Mouse(MouseEvent::Down(btn, x, y, _modifier)) => {
                    focused_widget_idx =
                        widget_node_idx_at(&layout_tree, x as f32, y as f32);

                    if let Some(idx) = focused_widget_idx.as_ref() {
                        let active_widget: Option<&mut dyn Widget> =
                            find_widget_mut(&mut root_node, *idx);
                        set_focused_node(&mut root_node, *idx);
                    }
                }
                _ => (),
            }
        }
    }
    finalize(w)?;
    Ok(())
}

fn draw_buffer<W: Write>(
    w: &mut W,
    root_node: &Widget,
    layout_tree: &LayoutTree,
    width: u16,
    height: u16,
) {
    let mut buf = Buffer::new(width as usize, height as usize);
    queue!(w, cursor::MoveTo(1, 1));
    let cmds = root_node.draw(&mut buf, &layout_tree);
    write!(w, "{}", buf);
    cmds.iter()
        .for_each(|cmd| cmd.execute(w).expect("must execute"));
    w.flush();
}

pub fn buffer_size() -> Result<(u16, u16)> {
    terminal::size()
}

fn main() -> Result<()> {
    let mut stderr = io::stdout();
    run(&mut stderr)
}
