pub mod client;
mod datastore;
pub mod error;
pub(crate) mod models;
pub mod routes;
mod schema;

pub use datastore::*;

//-----------------------------------------------------------------------------
// Constants
//-----------------------------------------------------------------------------

pub const DEFAULT_PORT: u16 = 5054;
pub const DEFAULT_ADDR: [u8; 4] = [127, 0, 0, 1];

//-----------------------------------------------------------------------------
// Type Definitions
//-----------------------------------------------------------------------------

pub type Result<T> = std::result::Result<T, error::AppError>;
