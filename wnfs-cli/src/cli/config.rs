use super::Config;
use anyhow::Result;
use chrono::Utc;
use std::{
    fs::{self, File},
    io::Write,
    rc::Rc,
};
use wnfs::{private::PrivateForest, root_tree::RootTree};
use wnfs_store::client::WnfsStore;

pub fn handle(config: Config) -> Result<()> {
    println!("Config: Writing configuration {:#?}", config);
    write_config_toml(&config)
}

fn write_config_toml(config: &Config) -> Result<()> {
    let mut file = File::create("config.toml")?;
    let toml = toml::to_string(config)?;
    file.write_all(toml.as_bytes())?;
    Ok(())
}

pub(crate) fn read_config_toml() -> Result<Config> {
    let toml_str = fs::read_to_string("config.toml")?;
    let config = toml::from_str(&toml_str)?;
    Ok(config)
}

pub(crate) async fn create_default_config(mut store: WnfsStore) -> Result<Config> {
    let forest = Rc::new(PrivateForest::default());
    let root_tree = RootTree::<_, _, Utc>::new(
        forest,
        store.clone(),
        rand::thread_rng(),
        Default::default(),
    )
    .await;

    Ok(Config {
        store_url: store.url.to_string(),
        root_tree_cid: root_tree.store(&mut store).await?.to_string(),
    })
}
