use crate::{
    error,
    error::InternalError,
    models::{NewStore, Store},
    schema::store,
    DataStore, Result,
};
use async_trait::async_trait;
use diesel::{prelude::*, Connection, PgConnection, QueryDsl, RunQueryDsl};
use libipld::cid::Version;
use multihash::{Code, MultihashDigest};
use once_cell::sync::Lazy;
use std::{env, mem};
use tokio::sync::Mutex;
use wnfs::ipld::{Cid, IpldCodec};

//----------------------------------------------------------------
// Globals
//----------------------------------------------------------------

static CONNECTION: Lazy<Mutex<PgConnection>> = Lazy::new(connect_db);

//----------------------------------------------------------------
// Types
//----------------------------------------------------------------

pub(crate) struct StoreDb;

//----------------------------------------------------------------
// Implementations
//----------------------------------------------------------------

#[async_trait]
impl DataStore for StoreDb {
    async fn get(name: Option<String>, cid: &Cid) -> Result<Vec<u8>> {
        let name = name.unwrap_or_else(|| "default".to_string());
        let mut conn = CONNECTION.lock().await;

        let mut results = store::table
            .filter(
                store::cid
                    .eq(cid.to_bytes())
                    .and(store::store_name.eq(name)),
            )
            .load::<Store>(&mut *conn)
            .map_err(error::anyhow)?;

        results
            .get_mut(0)
            .ok_or_else(|| error::anyhow(InternalError::NotFoundInStore))
            .map(|s| mem::take(&mut s.bytes))
    }

    async fn save(name: Option<String>, bytes: Vec<u8>, codec: IpldCodec) -> Result<Cid> {
        let store_name = name.unwrap_or_else(|| "default".to_string());
        let mut conn = CONNECTION.lock().await;

        let hash = Code::Sha2_256.digest(&bytes);
        let cid = Cid::new(Version::V1, codec.into(), hash).map_err(error::anyhow)?;

        diesel::insert_into(store::table)
            .values(NewStore {
                store_name,
                cid: cid.to_bytes(),
                bytes,
            })
            .get_result::<Store>(&mut *conn)
            .map_err(error::anyhow)?;

        Ok(cid)
    }
}

//----------------------------------------------------------------
// Functions
//----------------------------------------------------------------

pub fn connect_db() -> Mutex<PgConnection> {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let conn = PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    Mutex::new(conn)
}
