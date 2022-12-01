use crate::{
    routes::{GetBody, PutBody},
    DataStoreKind,
};
use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;
use std::{borrow::Cow, net::SocketAddr};
use wnfs::{
    ipld::{Cid, IpldCodec},
    BlockStore,
};

//------------------------------------------------------------------------------
// Types
//------------------------------------------------------------------------------

pub struct WnfsStore {
    addr: SocketAddr,
    store_name: Option<String>,
    datastore: DataStoreKind,
}

//------------------------------------------------------------------------------
// Implementations
//------------------------------------------------------------------------------

impl WnfsStore {
    pub fn new(addr: SocketAddr, store_name: Option<String>, datastore: DataStoreKind) -> Self {
        Self {
            addr,
            store_name,
            datastore,
        }
    }
}

#[async_trait(?Send)]
impl BlockStore for WnfsStore {
    async fn get_block<'a>(&'a self, cid: &Cid) -> Result<Cow<'a, Vec<u8>>> {
        let url = format!("{}/store", self.addr);
        let resp = Client::new()
            .get(url)
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
        let url = format!("{}/store", self.addr);
        let resp = Client::new()
            .put(url)
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
