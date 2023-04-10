use crate::{
    routes::{GetBody, PutBody},
    DataStoreKind,
};
use anyhow::Result;
use async_trait::async_trait;
use reqwest::{Client, Url};
use std::borrow::Cow;
use wnfs::{
    common::BlockStore,
    libipld::{Cid, IpldCodec},
};

//------------------------------------------------------------------------------
// Types
//------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct WnfsStore {
    pub url: Url,
    pub store_name: Option<String>,
    pub datastore: DataStoreKind,
}

//------------------------------------------------------------------------------
// Implementations
//------------------------------------------------------------------------------

impl WnfsStore {
    pub fn new(url: Url, store_name: Option<String>, datastore: DataStoreKind) -> Self {
        Self {
            url,
            store_name,
            datastore,
        }
    }
}

#[async_trait(?Send)]
impl BlockStore for WnfsStore {
    async fn get_block<'a>(&'a self, cid: &Cid) -> Result<Cow<'a, Vec<u8>>> {
        let resp = Client::new()
            .get(self.url.clone())
            .json(&GetBody {
                cid: *cid,
                store_name: self.store_name.clone(),
                datastore: self.datastore.into(),
            })
            .send()
            .await?;

        Ok(Cow::Owned(resp.bytes().await?.to_vec()))
    }

    async fn put_block(&mut self, bytes: Vec<u8>, codec: IpldCodec) -> Result<Cid> {
        let resp = Client::new()
            .put(self.url.clone())
            .json(&PutBody {
                bytes,
                codec: codec.into(),
                store_name: self.store_name.clone(),
                datastore: self.datastore.into(),
            })
            .send()
            .await?;

        let bytes = resp.bytes().await?;
        let cid = Cid::try_from(&bytes[..])?;
        Ok(cid)
    }
}
