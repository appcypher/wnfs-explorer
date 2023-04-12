use crate::{Border, Point, Size, Widget};
use anyhow::Result;
use crossterm::{
    cursor::{self, MoveDown, MoveLeft, MoveTo},
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    style::{self, Print, SetAttribute, SetBackgroundColor, SetForegroundColor},
    terminal::{self, disable_raw_mode, enable_raw_mode, Clear, ClearType},
    QueueableCommand,
};
use std::{
    cell::RefCell,
    io::{stdout, Stdout, Write},
    rc::Rc,
    time::Duration,
};
use unicode_segmentation::UnicodeSegmentation;

//------------------------------------------------------------------------------
// Constants
//------------------------------------------------------------------------------

const FOCUS_COLOR: style::Color = style::Color::Green;

//------------------------------------------------------------------------------
// Types
//------------------------------------------------------------------------------

pub struct TermContext {
    size: Size,
    cursor_position: Point,
    focus_widget: Option<Rc<RefCell<Widget>>>,
}

//------------------------------------------------------------------------------
// Implementations
//------------------------------------------------------------------------------

impl TermContext {
    pub fn new() -> Result<Self> {
        Ok(Self {
            size: terminal::size()?.into(),
            cursor_position: cursor::position()?.into(),
            focus_widget: None,
        })
    }

    pub fn event_loop(
        &mut self,
        root_widget: Rc<RefCell<Widget>>,
        parent_offset: Point,
        bounds: Size,
    ) -> Result<()> {
        enable_raw_mode()?;
        self.render(&root_widget, parent_offset, bounds)?;
        loop {
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(event) = event::read()? {
                    if event.code == KeyCode::Char('c') && event.modifiers == KeyModifiers::CONTROL
                    {
                        break;
                    }

                    if event.code == KeyCode::Tab {
                        self.set_next_focus(&root_widget)?;
                    }

                    self.render(&root_widget, parent_offset, bounds)?;

                    self.notify_focus_widget(event)?;
                }
            }
        }

        disable_raw_mode()?;

        println!();

        Ok(())
    }

    pub fn notify_focus_widget(&mut self, event: KeyEvent) -> Result<()> {
        if let Some(ref focus_widget) = self.focus_widget {
            if let Some(ref handler) = focus_widget.borrow().event_handler {
                handler(focus_widget, &event)?;
            }
        }

        Ok(())
    }

    pub fn set_next_focus(&mut self, root_widget: &Rc<RefCell<Widget>>) -> Result<()> {
        if let Some(focus_widget) = self.focus_widget.as_ref().map(Rc::clone) {
            let focus_widget_ref = focus_widget.borrow();
            if let Some(child) = focus_widget_ref.children.first() {
                self.focus_widget = Some(Rc::clone(child));
                return Ok(());
            }

            if let Some(ref parent) = focus_widget_ref.parent {
                if let Some(parent) = parent.upgrade() {
                    let parent_ref = parent.borrow();
                    let mut children = parent_ref.children.iter();
                    while let Some(child) = children.next() {
                        if Rc::ptr_eq(child, &focus_widget) {
                            if let Some(next_child) = children.next() {
                                self.focus_widget = Some(Rc::clone(next_child));
                                return Ok(());
                            }
                        }
                    }
                }
            } else {
                println!("no (parent): {:?}", focus_widget_ref.parent);
            }
        }

        self.focus_widget = Some(Rc::clone(root_widget));

        Ok(())
    }

    pub fn render(
        &mut self,
        widget: &Rc<RefCell<Widget>>,
        parent_offset: Point,
        bounds: Size,
    ) -> Result<()> {
        let out = &mut stdout();
        let widget_ref = widget.borrow();
        let total_offset = self.cursor_position + parent_offset + widget_ref.position;

        // Clear the screen.
        self.clear()?;

        // TODO(appcypher): We should check that the remaining space is enough to render the widget. We scroll if not.
        // if self.canvas_height() < widget.size.height { ... }

        // Render background.
        self.render_background(widget, total_offset)?;

        // Render text using specified color and style.
        self.render_text(widget, total_offset)?;

        // Render the border.
        if widget_ref.border.is_some() {
            self.render_border(widget, total_offset)?;
        }

        // Render the children.
        for child in &widget_ref.children {
            self.render(child, parent_offset + widget_ref.position, bounds)?;
        }

        // Set cursor position to corner of canvas to prevent flushing issue.
        self.set_to_corner(bounds, out)?;

        // Flush the output
        out.flush()?;

        // Update context size.
        self.size = widget_ref.size;

        Ok(())
    }

    pub fn render_background(
        &mut self,
        widget: &Rc<RefCell<Widget>>,
        total_offset: Point,
    ) -> Result<()> {
        let out = &mut stdout();
        let widget_ref = widget.borrow();

        match self.focus_widget {
            Some(ref focus_widget) if Rc::ptr_eq(focus_widget, widget) => {
                out.queue(SetBackgroundColor(FOCUS_COLOR))?;
                out.queue(SetForegroundColor(FOCUS_COLOR))?;
            }
            _ => {
                out.queue(SetBackgroundColor(widget_ref.background_color.inner()))?;
                out.queue(SetForegroundColor(widget_ref.background_color.inner()))?;
            }
        }

        for i in 0..widget_ref.size.height {
            out.queue(MoveTo(total_offset.x, total_offset.y + i))?;
            out.queue(Print("█".repeat(widget_ref.size.width as usize)))?;
        }

        out.queue(SetBackgroundColor(style::Color::Reset))?;
        out.queue(SetForegroundColor(style::Color::Reset))?;

        out.flush()?;

        Ok(())
    }

    pub fn render_border(
        &mut self,
        widget: &Rc<RefCell<Widget>>,
        total_offset: Point,
    ) -> Result<()> {
        let out = &mut stdout();
        let widget_ref = widget.borrow();
        let border = widget_ref.border.unwrap();

        match self.focus_widget {
            Some(ref focus_widget) if Rc::ptr_eq(focus_widget, widget) => {
                out.queue(SetBackgroundColor(FOCUS_COLOR))?;
            }
            _ => {
                out.queue(SetBackgroundColor(widget_ref.background_color.inner()))?;
            }
        }
        out.queue(SetForegroundColor(widget_ref.border_color.inner()))?;

        // Top border.
        if border.top {
            out.queue(MoveTo(total_offset.x + 1, total_offset.y))?;
            out.queue(Print("─".repeat((widget_ref.size.width - 2) as usize)))?;
        }

        // Left border.
        if border.left {
            out.queue(MoveTo(total_offset.x, total_offset.y))?;
            for _ in 0..widget_ref.size.height - 2 {
                out.queue(MoveDown(1))?;
                out.queue(Print("│"))?;
                out.queue(MoveLeft(1))?;
            }
        }

        // Right border.
        if border.right {
            out.queue(MoveTo(
                total_offset.x + widget_ref.size.width - 1,
                total_offset.y,
            ))?;
            for _ in 0..widget_ref.size.height - 2 {
                out.queue(MoveDown(1))?;
                out.queue(Print("│"))?;
                out.queue(MoveLeft(1))?;
            }
        }

        // Bottom border.
        if border.bottom {
            out.queue(MoveTo(
                total_offset.x + 1,
                total_offset.y + widget_ref.size.height - 1,
            ))?;
            out.queue(Print("─".repeat((widget_ref.size.width - 2) as usize)))?;
        }

        // Top left corner.
        out.queue(MoveTo(total_offset.x, total_offset.y))?;
        if border.top && border.left {
            out.queue(Print("┌"))?;
        } else if border.top {
            out.queue(Print("╶"))?;
        } else if border.left {
            out.queue(Print("╷"))?;
        }

        // Top right corner.
        out.queue(MoveTo(
            total_offset.x + widget_ref.size.width - 1,
            total_offset.y,
        ))?;
        if border.top && border.right {
            out.queue(Print("┐"))?;
        } else if border.top {
            out.queue(Print("╴"))?;
        } else if border.right {
            out.queue(Print("╷"))?;
        }

        // Bottom left corner.
        out.queue(MoveTo(
            total_offset.x,
            total_offset.y + widget_ref.size.height - 1,
        ))?;
        if border.bottom && border.left {
            out.queue(Print("└"))?;
        } else if border.bottom {
            out.queue(Print("╶"))?;
        } else if border.left {
            out.queue(Print("╵"))?;
        }

        // Bottom right corner.
        out.queue(MoveTo(
            total_offset.x + widget_ref.size.width - 1,
            total_offset.y + widget_ref.size.height - 1,
        ))?;
        if border.bottom && border.right {
            out.queue(Print("┘"))?;
        } else if border.bottom {
            out.queue(Print("╴"))?;
        } else if border.right {
            out.queue(Print("╵"))?;
        }

        out.queue(SetBackgroundColor(style::Color::Reset))?;
        out.queue(SetForegroundColor(style::Color::Reset))?;

        out.flush()?;

        Ok(())
    }

    // TODO(appcypher): Support other text_wrapping modes.
    pub fn render_text(&mut self, widget: &Rc<RefCell<Widget>>, total_offset: Point) -> Result<()> {
        let out = &mut stdout();
        let widget_ref = widget.borrow();
        let border = widget_ref.border.unwrap_or(Border::default());

        // Avoid rendering on borders if present.
        let total_offset = total_offset + Point::new(border.left as u16, border.top as u16);
        let text: String = UnicodeSegmentation::graphemes(widget_ref.text.as_str(), true)
            .take((widget_ref.size.width - (border.left as u16 + border.right as u16)) as usize)
            .collect();

        match self.focus_widget {
            Some(ref focus_widget) if Rc::ptr_eq(focus_widget, widget) => {
                out.queue(SetBackgroundColor(FOCUS_COLOR))?;
            }
            _ => {
                out.queue(SetBackgroundColor(widget_ref.background_color.inner()))?;
            }
        }
        out.queue(SetForegroundColor(widget_ref.border_color.inner()))?;
        out.queue(SetAttribute(widget_ref.text_style.inner()))?;

        out.queue(MoveTo(total_offset.x, total_offset.y))?;
        out.queue(Print(&text))?;

        out.queue(SetBackgroundColor(style::Color::Reset))?;
        out.queue(SetForegroundColor(style::Color::Reset))?;
        out.queue(SetAttribute(style::Attribute::Reset))?;

        out.flush()?;

        Ok(())
    }

    pub fn set_to_corner(&mut self, bounds: Size, out: &mut Stdout) -> Result<()> {
        let offset = self.cursor_position + bounds.into();
        out.queue(MoveTo(offset.x, offset.y))?;
        Ok(())
    }

    pub fn canvas_height(&self) -> u16 {
        self.size.height - self.cursor_position.y
    }

    pub fn clear(&mut self) -> Result<()> {
        let out = &mut stdout();

        for y in self.cursor_position.y..self.size.height {
            out.queue(MoveTo(0, y))?;
            out.queue(Clear(ClearType::CurrentLine))?;
        }

        out.flush()?;

        self.size = Size::new(0, 0);

        Ok(())
    }
}
