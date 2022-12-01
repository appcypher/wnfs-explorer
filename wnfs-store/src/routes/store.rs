use crate::Result;
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
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PutBody {
    pub bytes: Vec<u8>,
    pub codec: IpldCodecRepr,
    pub store_name: Option<String>,
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

impl From<IpldCodec> for IpldCodecRepr {
    fn from(codec: IpldCodec) -> Self {
        match codec {
            IpldCodec::Raw => Self::Raw,
            IpldCodec::DagCbor => Self::DagCbor,
            IpldCodec::DagJson => Self::DagJson,
            IpldCodec::DagPb => Self::DagPb,
        }
    }
}

//----------------------------------------------------------------
// Functions
//----------------------------------------------------------------

pub async fn get_from_store(Json(_body): Json<GetBody>) -> Result<impl IntoResponse> {
    Ok("Nothing yet: get_from_store")
}

pub async fn put_in_store(Json(_body): Json<PutBody>) -> Result<impl IntoResponse> {
    Ok("Nothing yet: put_in_store")
}
