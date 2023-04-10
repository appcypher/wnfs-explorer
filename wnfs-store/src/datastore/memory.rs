use crate::{
    error::{self, InternalError},
    DataStore, Result,
};
use async_trait::async_trait;
use hashbrown::HashMap;
use lazy_static::lazy_static;
use tokio::sync::Mutex;
use wnfs::libipld::{
    multihash::{Code, MultihashDigest},
    Cid, IpldCodec,
};

//----------------------------------------------------------------
// Globals
//----------------------------------------------------------------

lazy_static! {
    static ref STORES: Mutex<HashMap<String, Store>> = Mutex::new(HashMap::new());
}

//----------------------------------------------------------------
// Types
//----------------------------------------------------------------

pub(crate) struct StoreMem;

type Store = HashMap<Cid, Vec<u8>>;

//----------------------------------------------------------------
// Implementations
//----------------------------------------------------------------

#[async_trait]
impl DataStore for StoreMem {
    async fn get(name: Option<String>, cid: &Cid) -> Result<Vec<u8>> {
        let name = name.unwrap_or_else(|| "default".to_string());

        let mut stores = STORES.lock().await;
        let store = stores.entry(name).or_default();

        let bytes = store
            .get(cid)
            .cloned()
            .ok_or_else(|| error::anyhow(InternalError::NotFoundInStore))?;

        Ok(bytes)
    }

    async fn save(name: Option<String>, bytes: Vec<u8>, codec: IpldCodec) -> Result<Cid> {
        let name = name.unwrap_or_else(|| "default".to_string());

        let mut stores = STORES.lock().await;
        let store = stores.entry(name).or_default();

        let hash = Code::Sha2_256.digest(&bytes);
        let cid = Cid::new_v1(codec.into(), hash);

        store.insert(cid, bytes);

        Ok(cid)
    }
}
