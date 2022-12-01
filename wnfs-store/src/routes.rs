use crate::{db::StoreDb, memory::StoreMem, DataStore, DataStoreKind, Result};
use axum::{response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use wnfs::ipld::{Cid, IpldCodec};

//----------------------------------------------------------------
// Types
//----------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug)]
pub struct GetBody {
    pub cid: Cid,
    pub store_name: Option<String>,
    pub datastore: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PutBody {
    pub bytes: Vec<u8>,
    pub codec: IpldCodecRepr,
    pub store_name: Option<String>,
    pub datastore: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[repr(C)]
pub enum IpldCodecRepr {
    Raw = 0x00,
    DagCbor = 0x01,
    DagJson = 0x02,
    DagPb = 0x03,
}

//----------------------------------------------------------------
// Implementations
//----------------------------------------------------------------

impl From<IpldCodecRepr> for IpldCodec {
    fn from(repr: IpldCodecRepr) -> Self {
        match repr {
            IpldCodecRepr::Raw => IpldCodec::Raw,
            IpldCodecRepr::DagCbor => IpldCodec::DagCbor,
            IpldCodecRepr::DagJson => IpldCodec::DagJson,
            IpldCodecRepr::DagPb => IpldCodec::DagPb,
        }
    }
}

impl From<IpldCodec> for IpldCodecRepr {
    fn from(codec: IpldCodec) -> Self {
        match codec {
            IpldCodec::Raw => IpldCodecRepr::Raw,
            IpldCodec::DagCbor => IpldCodecRepr::DagCbor,
            IpldCodec::DagJson => IpldCodecRepr::DagJson,
            IpldCodec::DagPb => IpldCodecRepr::DagPb,
        }
    }
}

//----------------------------------------------------------------
// Functions
//----------------------------------------------------------------

pub async fn get_from_store(Json(body): Json<GetBody>) -> Result<impl IntoResponse> {
    let kind: DataStoreKind = body.datastore.try_into()?;
    let bytes = match kind {
        DataStoreKind::Memory => StoreMem::get(body.store_name, &body.cid).await?,
        DataStoreKind::Db => StoreDb::get(body.store_name, &body.cid).await?,
    };

    Ok(bytes)
}

pub async fn put_in_store(Json(body): Json<PutBody>) -> Result<impl IntoResponse> {
    let kind: DataStoreKind = body.datastore.try_into()?;
    let cid = match kind {
        DataStoreKind::Memory => {
            StoreMem::save(body.store_name, body.bytes, body.codec.into()).await?
        }
        DataStoreKind::Db => StoreDb::save(body.store_name, body.bytes, body.codec.into()).await?,
    };

    Ok(cid.to_bytes())
}
