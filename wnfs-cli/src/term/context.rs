use crate::{Border, Point, Size, Widget};
use anyhow::Result;
use crossterm::{
    cursor::{self, MoveDown, MoveLeft, MoveTo},
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    style::{self, Print, SetAttribute, SetBackgroundColor, SetForegroundColor},
    terminal::{self, disable_raw_mode, enable_raw_mode, Clear, ClearType, ScrollDown, ScrollUp},
    ExecutableCommand, QueueableCommand,
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

// TODO(appcypher): Fix whitespace artefacts.
// TODO(appcypher): Implement scrolling.
// TODO(appcypher): Implement modal widgets.
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
        self.clear()?;
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

                    self.clear()?;
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
                if child.borrow().focusable {
                    self.focus_widget = Some(Rc::clone(child));
                    return Ok(());
                }
            }

            if let Some(ref parent) = focus_widget_ref.parent {
                if let Some(parent) = parent.upgrade() {
                    let parent_ref = parent.borrow();
                    let mut children = parent_ref.children.iter();
                    while let Some(child) = children.next() {
                        if Rc::ptr_eq(child, &focus_widget) {
                            if let Some(next_child) = children.next() {
                                if next_child.borrow().focusable {
                                    self.focus_widget = Some(Rc::clone(next_child));
                                    return Ok(());
                                }
                            }
                        }
                    }
                }
            }
        }

        if root_widget.borrow().focusable {
            self.focus_widget = Some(Rc::clone(root_widget));
        }

        Ok(())
    }

    pub fn render(
        &mut self,
        widget: &Rc<RefCell<Widget>>,
        parent_offset: Point,
        parent_bounds: Size,
    ) -> Result<()> {
        let widget_ref = widget.borrow();

        // TODO(appcypher): We should make room for rendering the widget.
        // self.make_room(parent_bounds)?;

        // Calculate the total offset of the widget.
        let total_offset = self.calculate_total_offset(parent_offset, widget_ref.position);

        // TODO(appcypher): Consider border of parents in clipped bounds.
        let clipped_bounds = Size::new(
            u16::min(
                (parent_offset.x + parent_bounds.width),
                widget_ref.size.width,
            ),
            u16::min(
                total_offset.y - (parent_offset.y + parent_bounds.height),
                widget_ref.size.height,
            ),
        );

        // Render background.
        self.render_background(widget, total_offset, clipped_bounds)?;

        // // Render text using specified color and style.
        // self.render_text(widget, total_offset)?;

        // // Render the border.
        // if widget_ref.border.is_some() {
        //     self.render_border(widget, total_offset)?;
        // }

        // Render the children.
        for child in &widget_ref.children {
            self.render(child, parent_offset + widget_ref.position, clipped_bounds)?;
        }

        // Set cursor position to corner of canvas to prevent flushing issue.
        self.set_to_corner(parent_bounds)?;

        Ok(())
    }

    pub fn calculate_total_offset(&self, parent_offset: Point, widget_position: Point) -> Point {
        self.cursor_position + parent_offset + widget_position
    }

    pub fn render_background(
        &mut self,
        widget: &Rc<RefCell<Widget>>,
        total_offset: Point,
        clipped_bounds: Size,
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

        for i in 0..clipped_bounds.height {
            out.queue(MoveTo(total_offset.x, total_offset.y + i))?;
            out.queue(Print("█".repeat(clipped_bounds.width as usize)))?;
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

    pub fn set_to_corner(&mut self, bounds: Size) -> Result<()> {
        let out = &mut stdout();
        let offset = self.cursor_position + bounds.into();
        out.queue(MoveTo(offset.x, offset.y))?;
        out.flush()?;
        Ok(())
    }

    pub fn canvas_height(&self) -> u16 {
        self.size.height - self.cursor_position.y
    }

    pub fn make_room(&mut self, bounds: Size) -> Result<()> {
        let canvas_height = self.canvas_height();
        if canvas_height < bounds.height + 1 {
            let out = &mut stdout();
            out.queue(MoveTo(0, self.cursor_position.y))?;
            out.queue(ScrollUp(bounds.height - canvas_height))?;
            out.queue(MoveTo(0, self.cursor_position.y - bounds.height + 1))?;
            out.flush()?;

            self.size = terminal::size()?.into();
            self.cursor_position = cursor::position()?.into();
        }

        Ok(())
    }

    pub fn clear(&mut self) -> Result<()> {
        let out = &mut stdout();

        for y in self.cursor_position.y..self.size.height {
            out.queue(MoveTo(0, y))?;
            out.queue(Clear(ClearType::CurrentLine))?;
        }

        out.queue(MoveTo(0, self.cursor_position.y))?;
        out.flush()?;

        Ok(())
    }
}
