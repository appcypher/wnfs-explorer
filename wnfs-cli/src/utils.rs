use crate::{Color, Point, Size, Widget};
use anyhow::Result;
use std::{cell::RefCell, rc::Rc};

//------------------------------------------------------------------------------
// Functions
//------------------------------------------------------------------------------

pub(crate) fn sample_widget_1() -> Result<Rc<RefCell<Widget>>> {
    let a = Rc::new(RefCell::new(Widget {
        position: Point::new(1, 2),
        size: Size::new(5, 5),
        text: "A".to_string(),
        ..Default::default()
    }));

    let b = Rc::new(RefCell::new(Widget {
        position: Point::new(6, 2),
        size: Size::new(5, 5),
        text: "B".to_string(),
        ..Default::default()
    }));

    let root = Rc::new(RefCell::new(Widget {
        size: Size::new(14, 10),
        text: "Hello there my world!".to_string(),
        background_color: Color::black(),
        text_color: Color::red(),
        border_color: Color::red(),
        text_style: crate::TextStyle::bold(),
        ..Default::default()
    }));

    Widget::add_child(&root, &a)?;
    Widget::add_child(&root, &b)?;
    Widget::add_event_handler(&root, Box::new(|_, _| Ok(())));

    Ok(root)
}

pub(crate) fn sample_widget_2() -> Result<Rc<RefCell<Widget>>> {
    let a = Rc::new(RefCell::new(Widget {
        position: Point::new(1, 2),
        size: Size::new(5, 10),
        text: "A".to_string(),
        background_color: Color::yellow(),
        ..Default::default()
    }));

    let b = Rc::new(RefCell::new(Widget {
        position: Point::new(6, 2),
        size: Size::new(10, 10),
        text: "B".to_string(),
        background_color: Color::blue(),
        ..Default::default()
    }));

    let root = Rc::new(RefCell::new(Widget {
        size: Size::new(14, 10),
        text: "Hello there my world!".to_string(),
        background_color: Color::black(),
        text_color: Color::red(),
        border_color: Color::red(),
        text_style: crate::TextStyle::bold(),
        ..Default::default()
    }));

    Widget::add_child(&root, &a)?;
    Widget::add_child(&root, &b)?;
    Widget::add_event_handler(&root, Box::new(|_, _| Ok(())));

    Ok(root)
}
