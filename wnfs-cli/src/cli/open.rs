use super::Noun;
use crate::{config, utils, Point, TermContext};
use anyhow::Result;
use reqwest::Url;
use std::net::SocketAddr;
use wnfs_store::{client::WnfsStore, DEFAULT_ADDR, DEFAULT_PORT};

//------------------------------------------------------------------------------
// Functions
//------------------------------------------------------------------------------

pub async fn handle(_noun: Noun) -> Result<()> {
    let (_config, _store) = match config::read_config_toml() {
        Ok(config) => {
            let store = WnfsStore::new(
                Url::parse(&config.store_url)?,
                None,
                wnfs_store::DataStoreKind::Memory,
            );
            (config, store)
        }
        Err(_) => {
            let store = WnfsStore::new(
                Url::parse(&format!(
                    "http://{}",
                    SocketAddr::from((DEFAULT_ADDR, DEFAULT_PORT))
                ))?,
                None,
                wnfs_store::DataStoreKind::Memory,
            );
            let config = config::create_default_config(store.clone()).await?;
            (config, store)
        }
    };

    // Create terminal context.
    let mut context = TermContext::new()?;

    // Create sample widget.
    let widget = utils::sample_widget()?;
    let widget_size = widget.borrow().size;

    // Event loop.
    context.event_loop(widget, Point::default(), widget_size)?;

    Ok(())
}
