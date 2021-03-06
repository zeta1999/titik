//! Provides the core functionality of rendering to the terminal
//! This has the event loop which calculates and process the events to the target widget
use crate::{
    command,
    find_node,
    layout,
    Buffer,
    LayoutTree,
    Widget,
};
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
use std::io::Write;
use stretch::{
    geometry::Size,
    number::Number,
};

/// A Dispatch trait which the implementing APP will update
/// its own state based on the supplied msg.
pub trait Dispatch<MSG> {
    /// dispatch the msg and passed the root node for the implementing
    /// app to access it and change the state of the UI.
    fn dispatch(&self, msg: MSG, root_node: &mut dyn Widget<MSG>);
}

/// This provides the render loop of the terminal UI
pub struct Renderer<'a, MSG> {
    write: &'a mut dyn Write,
    program: Option<&'a dyn Dispatch<MSG>>,
    root_node: &'a mut dyn Widget<MSG>,
    layout_tree: LayoutTree,
    terminal_size: (u16, u16),
    focused_widget_idx: Option<usize>,
}

impl<'a, MSG> Renderer<'a, MSG> {
    /// create a new renderer with the supplied root_node
    pub fn new(
        write: &'a mut dyn Write,
        program: Option<&'a dyn Dispatch<MSG>>,
        root_node: &'a mut dyn Widget<MSG>,
    ) -> Self {
        let (width, height) =
            terminal::size().expect("must get the terminal size");

        root_node.set_size(Some((width) as f32), Some(height as f32));

        let layout_tree = layout::compute_layout(
            root_node,
            Size {
                width: Number::Defined(width as f32),
                height: Number::Defined(height as f32),
            },
        );
        Renderer {
            write,
            program,
            root_node,
            layout_tree,
            terminal_size: (width, height),
            focused_widget_idx: None,
        }
    }

    fn recompute_layout(&mut self) {
        let (width, height) = self.terminal_size;
        self.layout_tree = layout::compute_layout(
            self.root_node,
            Size {
                width: Number::Defined(width as f32),
                height: Number::Defined(height as f32),
            },
        );
    }

    fn dispatch_msg(&mut self, msgs: Vec<MSG>) {
        if let Some(program) = self.program {
            for msg in msgs {
                program.dispatch(msg, self.root_node);
            }
        }
        self.recompute_layout();
    }

    /// run the event loop of the renderer
    pub fn run(&mut self) -> Result<()> {
        command::init(&mut self.write)?;
        command::reset_top(&mut self.write)?;
        let (width, height) = self.terminal_size;

        loop {
            let mut buf = Buffer::new(width as usize, height as usize);

            buf.reset();
            {
                let tty_cmds = self.root_node.draw(&mut buf, &self.layout_tree);
                buf.render(self.write)?;
                tty_cmds.iter().for_each(|cmd| {
                    cmd.execute(self.write).expect("must execute")
                });
            }
            self.write.flush()?;

            if let Ok(event) = event::read() {
                match event {
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
                            // send the keypresses to the focused widget
                            if let Some(idx) = self.focused_widget_idx.as_ref()
                            {
                                let active_widget: Option<
                                    &mut dyn Widget<MSG>,
                                > = find_node::find_widget_mut(
                                    self.root_node,
                                    *idx,
                                );
                                if let Some(focused_widget) = active_widget {
                                    let msgs =
                                        focused_widget.process_event(event);
                                    self.dispatch_msg(msgs);
                                }
                            }
                        }
                    }
                    // mouse clicks sets the focused the widget underneath
                    Event::Mouse(MouseEvent::Down(_btn, x, y, _modifier)) => {
                        self.focused_widget_idx = layout::widget_node_idx_at(
                            &self.layout_tree,
                            x as f32,
                            y as f32,
                        );

                        if let Some(idx) = self.focused_widget_idx.as_ref() {
                            layout::set_focused_node(self.root_node, *idx);
                        }
                    }
                    Event::Resize(width, height) => {
                        self.terminal_size = (width, height);
                        self.recompute_layout();
                    }
                    _ => (),
                }
                // any other activities, such as mouse scroll is
                // sent the widget underneath the location, regardless
                // if it focused or not.
                if let Some((x, y)) = extract_location(&event) {
                    let hits = self.layout_tree.hit(x as f32, y as f32);
                    //let hit = hits.pop().expect("process only 1 for now");
                    for hit in hits.iter().rev() {
                        let mut hit_widget: Option<&mut dyn Widget<MSG>> =
                            find_node::find_widget_mut(self.root_node, *hit);

                        if let Some(hit_widget) = &mut hit_widget {
                            let msgs = hit_widget.process_event(event);
                            self.dispatch_msg(msgs);
                        }
                    }
                }
            }
        }
        command::finalize(self.write)?;
        Ok(())
    }
}

/// extract the x and y location of a mouse event
fn extract_location(event: &Event) -> Option<(u16, u16)> {
    match event {
        Event::Mouse(MouseEvent::Down(_btn, x, y, _modifier)) => Some((*x, *y)),
        Event::Mouse(MouseEvent::Up(_btn, x, y, _modifier)) => Some((*x, *y)),
        Event::Mouse(MouseEvent::Drag(_btn, x, y, _modifier)) => Some((*x, *y)),
        Event::Mouse(MouseEvent::ScrollDown(x, y, _modifier)) => Some((*x, *y)),
        Event::Mouse(MouseEvent::ScrollUp(x, y, _modifier)) => Some((*x, *y)),
        Event::Key(_) => None,
        Event::Resize(_, _) => None,
    }
}
