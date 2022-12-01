use crate::routes::store::{GetBody, PutBody};
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
}

//------------------------------------------------------------------------------
// Implementations
//------------------------------------------------------------------------------

impl WnfsStore {
    pub fn new(addr: SocketAddr, store_name: Option<String>) -> Self {
        Self { addr, store_name }
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
            })
            .send()
            .await?;

        let bytes = resp.bytes().await?;
        let cid = Cid::try_from(&bytes[..])?;
        Ok(cid)
    }
}
