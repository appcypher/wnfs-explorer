use crate::schema::store;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct Store {
    pub id: i32,
    pub store_name: String,
    pub cid: Vec<u8>,
    pub bytes: Vec<u8>,
}

#[derive(Insertable)]
#[diesel(table_name = store)]
pub struct NewStore {
    pub store_name: String,
    pub cid: Vec<u8>,
    pub bytes: Vec<u8>,
}
