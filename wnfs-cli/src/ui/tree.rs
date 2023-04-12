use crate::{Widget};
use anyhow::Result;

//------------------------------------------------------------------------------
// Types
//------------------------------------------------------------------------------

struct TreeState {
    opened: Vec<String>,
    selected: Option<String>, // TODO(appcypher): Is this really needed.
}

pub type TreeWidget = Widget;

//------------------------------------------------------------------------------
// Implementations
//------------------------------------------------------------------------------

impl TreeWidget {
    pub fn new(// store: &WnfsStore,
        // config: &Config,
    ) -> Result<Self> {
        let state = TreeState::new();
        let tree = Widget {
            // size: Size::new(14, 10),
            // text: "Hello there my world!".to_string(),
            // background_color: Color::black(),
            // text_color: Color::red(),
            // border_color: Color::red(),
            // text_style: TextStyle::bold(),
            ..Default::default()
        };

        Ok(tree)
    }
}

impl TreeState {
    pub fn new() -> Self {
        Self {
            opened: Vec::new(),
            selected: None,
        }
    }
}
