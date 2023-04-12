use thiserror::Error;

//----------------------------------------------------------------
// Types
//----------------------------------------------------------------

#[derive(Debug, Clone, Copy, Error)]
pub enum WidgetError {
    #[error("Already has a parent")]
    HasParent,
}
