pub mod db;
pub mod memory;

//----------------------------------------------------------------
// Imports
//----------------------------------------------------------------

use crate::{
    error::{self, AppError, InternalError},
    Result,
};
use async_trait::async_trait;
use wnfs::ipld::{Cid, IpldCodec};

//----------------------------------------------------------------
// Types
//----------------------------------------------------------------

#[async_trait]
pub(super) trait DataStore {
    async fn get(name: Option<String>, cid: &Cid) -> Result<Vec<u8>>;
    async fn save(name: Option<String>, bytes: Vec<u8>, codec: IpldCodec) -> Result<Cid>;
}

#[derive(Debug, Clone, Copy)]
pub enum DataStoreKind {
    Memory,
    Db,
}

//----------------------------------------------------------------
// Implementations
//----------------------------------------------------------------

impl From<DataStoreKind> for String {
    fn from(kind: DataStoreKind) -> Self {
        match kind {
            DataStoreKind::Memory => "mem".to_string(),
            DataStoreKind::Db => "db".to_string(),
        }
    }
}

impl TryFrom<String> for DataStoreKind {
    type Error = AppError;

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        match value.as_str() {
            "mem" => Ok(DataStoreKind::Memory),
            "db" => Ok(DataStoreKind::Db),
            _ => Err(error::anyhow(InternalError::InvalidDataStoreKind(value))),
        }
    }
}
