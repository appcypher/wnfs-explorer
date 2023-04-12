use crate::WidgetError;
use anyhow::{bail, Result};
use crossterm::{event::KeyEvent, style};
use std::{
    cell::RefCell,
    fmt::{Debug, Formatter},
    ops::Add,
    rc::{Rc, Weak},
};

//------------------------------------------------------------------------------
// Types
//------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
pub enum TextWrap {
    Hidden,
    Wrap,
    Ellipsis,
    Overflow,
}

#[derive(Debug, Clone)]
pub enum BorderStyle {
    None,
    Single,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Border {
    pub top: bool,
    pub bottom: bool,
    pub left: bool,
    pub right: bool,
}

#[derive(Debug, Clone)]
pub struct Color(style::Color);

#[derive(Debug, Clone, Copy)]
pub struct TextStyle(style::Attribute);

pub struct Widget {
    pub position: Point,
    pub size: Size,
    pub parent: Option<Weak<RefCell<Widget>>>,
    pub children: Vec<Rc<RefCell<Widget>>>,
    pub border: Option<Border>,
    pub text_wrap: TextWrap,
    pub text: String,
    pub content_editable: bool,
    pub focusable: bool,
    pub focus_color: Option<Color>,
    pub background_color: Color,
    pub text_color: Color,
    pub text_style: TextStyle,
    pub border_color: Color,
    pub border_style: BorderStyle,
    pub event_handler: Option<KeyEventHandler>,
}

pub type KeyEventHandler = Box<dyn Fn(&Rc<RefCell<Widget>>, &KeyEvent) -> Result<()>>; // -> impl Future<Output = Result<()>>;

//------------------------------------------------------------------------------
// Implementations
//------------------------------------------------------------------------------

impl Widget {
    pub fn add_child(this: &Rc<RefCell<Self>>, child: &Rc<RefCell<Self>>) -> Result<()> {
        let mut child_ref_mut = child.borrow_mut();
        let mut this_ref_mut = this.borrow_mut();

        if child_ref_mut.parent.is_some() {
            bail!(WidgetError::HasParent);
        }

        child_ref_mut.parent = Some(Rc::downgrade(this));
        this_ref_mut.children.push(Rc::clone(child));

        Ok(())
    }

    pub fn add_event_handler(this: &Rc<RefCell<Self>>, handler: KeyEventHandler) {
        let mut this_ref_mut = this.borrow_mut();
        this_ref_mut.event_handler = Some(handler);
    }
}

impl Color {
    pub fn black() -> Self {
        Self(style::Color::Black)
    }

    pub fn red() -> Self {
        Self(style::Color::Red)
    }

    pub fn green() -> Self {
        Self(style::Color::Green)
    }

    pub fn white() -> Self {
        Self(style::Color::White)
    }

    pub fn blue() -> Self {
        Self(style::Color::Blue)
    }

    pub fn reset() -> Self {
        Self(style::Color::Reset)
    }

    pub fn inner(&self) -> style::Color {
        self.0
    }
}

impl TextStyle {
    pub fn underlined() -> Self {
        Self(style::Attribute::Underlined)
    }

    pub fn italic() -> Self {
        Self(style::Attribute::Italic)
    }

    pub fn bold() -> Self {
        Self(style::Attribute::Bold)
    }

    pub fn no_hidden() -> Self {
        Self(style::Attribute::NoHidden)
    }

    pub fn inner(&self) -> style::Attribute {
        self.0
    }
}

impl Border {
    pub fn new(top: bool, bottom: bool, left: bool, right: bool) -> Self {
        Self {
            top,
            bottom,
            left,
            right,
        }
    }

    pub fn all() -> Self {
        Self {
            top: true,
            bottom: true,
            left: true,
            right: true,
        }
    }

    pub fn trb() -> Self {
        Self {
            top: true,
            bottom: true,
            left: false,
            right: true,
        }
    }

    pub fn tbl() -> Self {
        Self {
            top: true,
            bottom: true,
            left: true,
            right: false,
        }
    }

    pub fn trl() -> Self {
        Self {
            top: true,
            bottom: false,
            left: true,
            right: true,
        }
    }

    pub fn brl() -> Self {
        Self {
            top: false,
            bottom: true,
            left: true,
            right: true,
        }
    }

    pub fn tr() -> Self {
        Self {
            top: true,
            bottom: false,
            left: false,
            right: true,
        }
    }

    pub fn tl() -> Self {
        Self {
            top: true,
            bottom: false,
            left: true,
            right: false,
        }
    }

    pub fn br() -> Self {
        Self {
            top: false,
            bottom: true,
            left: false,
            right: true,
        }
    }

    pub fn bl() -> Self {
        Self {
            top: false,
            bottom: true,
            left: true,
            right: false,
        }
    }

    pub fn top() -> Self {
        Self {
            top: true,
            bottom: false,
            left: false,
            right: false,
        }
    }

    pub fn bottom() -> Self {
        Self {
            top: false,
            bottom: true,
            left: false,
            right: false,
        }
    }

    pub fn left() -> Self {
        Self {
            top: false,
            bottom: false,
            left: true,
            right: false,
        }
    }

    pub fn right() -> Self {
        Self {
            top: false,
            bottom: false,
            left: false,
            right: true,
        }
    }
}

impl Size {
    pub fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }
}

impl Point {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add for &Point {
    type Output = Point;

    fn add(self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Default for BorderStyle {
    fn default() -> Self {
        Self::None
    }
}

impl Default for Color {
    fn default() -> Self {
        Self(style::Color::Reset)
    }
}

impl Default for TextStyle {
    fn default() -> Self {
        Self(style::Attribute::Reset)
    }
}

impl Default for TextWrap {
    fn default() -> Self {
        Self::Hidden
    }
}

impl From<(u16, u16)> for Point {
    fn from((x, y): (u16, u16)) -> Self {
        Self { x, y }
    }
}

impl From<(u16, u16)> for Size {
    fn from((width, height): (u16, u16)) -> Self {
        Self { width, height }
    }
}

impl From<Size> for Point {
    fn from(size: Size) -> Self {
        Self {
            x: size.width,
            y: size.height,
        }
    }
}

impl Default for Widget {
    fn default() -> Self {
        Self {
            position: Point::default(),
            size: Size::default(),
            parent: None,
            children: Vec::new(),
            border: None,
            text_wrap: TextWrap::default(),
            text: String::new(),
            content_editable: false,
            focusable: false,
            focus_color: None,
            background_color: Color::white(),
            text_color: Color::black(),
            text_style: TextStyle::no_hidden(),
            border_color: Color::black(),
            border_style: BorderStyle::default(),
            event_handler: None,
        }
    }
}

impl Debug for Widget {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Widget")
            .field("position", &self.position)
            .field("size", &self.size)
            .field("parent", &self.parent)
            .field("children", &self.children)
            .field("border", &self.border)
            .field("text_wrap", &self.text_wrap)
            .field("text", &self.text)
            .field("content_editable", &self.content_editable)
            .field("focusable", &self.focusable)
            .field("focus_color", &self.focus_color)
            .field("background_color", &self.background_color)
            .field("text_color", &self.text_color)
            .field("text_style", &self.text_style)
            .field("border_color", &self.border_color)
            .field("border_style", &self.border_style)
            .finish()
    }
}

