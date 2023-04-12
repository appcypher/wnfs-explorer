use std::{cell::RefCell, rc::Rc};

use crate::{Point, Widget};
use anyhow::Result;

//------------------------------------------------------------------------------
// Functions
//------------------------------------------------------------------------------

pub(crate) fn sample_widget() -> Result<Rc<RefCell<Widget>>> {
    let a = Rc::new(RefCell::new(Widget {
        position: Point::new(1, 2),
        size: crate::Size::new(5, 5),
        text: "A".to_string(),
        ..Default::default()
    }));

    let b = Rc::new(RefCell::new(Widget {
        position: Point::new(6, 2),
        size: crate::Size::new(5, 5),
        text: "B".to_string(),
        ..Default::default()
    }));

    let root = Rc::new(RefCell::new(Widget {
        size: crate::Size::new(14, 10),
        text: "Hello there my world!".to_string(),
        background_color: crate::Color::black(),
        text_color: crate::Color::red(),
        border_color: crate::Color::red(),
        text_style: crate::TextStyle::bold(),
        ..Default::default()
    }));

    Widget::add_child(&root, &a)?;
    Widget::add_child(&root, &b)?;
    Widget::add_event_handler(
        &root,
        Box::new(|_, event| {
            println!("Event: {:?}", event);
            Ok(())
        }),
    );

    Ok(root)
}
