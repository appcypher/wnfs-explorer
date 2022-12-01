use std::{collections::HashMap, rc::Rc};
use wnfs::{PrivateDirectory, PrivateNode, PublicDirectory};

pub type Root<T> = (String, Rc<T>);

#[derive(Debug)]
pub struct PrivateFileSystem {
    pub root: Root<PrivateDirectory>,
    pub hamt_name: String,
    pub store_name: String,
    pub map: HashMap<String, PrivateNode>,
}

#[derive(Debug)]
pub struct PublicFileSystem {
    pub root: Root<PublicDirectory>,
    pub store_name: String,
    pub map: HashMap<String, PrivateNode>,
}
