use anyhow::Result;
use crossterm::{
    cursor::{self, MoveTo},
    queue,
    style::{self, Stylize},
    terminal::{self, Clear, ClearType},
    QueueableCommand,
};
use reqwest::Url;
use std::{
    io::{stdout, Write},
    net::SocketAddr,
};
use wnfs::libipld::store;
use wnfs_store::{client::WnfsStore, DEFAULT_ADDR, DEFAULT_PORT};

use crate::config;

use super::Noun;

// pub(crate) fn handle(noun: Noun) -> Result<()> {
//     let mut stdout = stdout();

//     //-------------------------------------------------------------------------

//     let (col_size, row_size) = terminal::size()?;

//     for y in 0..row_size {
//         for x in 0..col_size {
//             if (y == 0 || y == row_size - 1) || (x == 0 || x == col_size - 1) {
//                 queue!(
//                     stdout,
//                     cursor::MoveTo(x, y),
//                     style::PrintStyledContent("█".yellow())
//                 )?;
//             }
//         }
//     }

//     //-------------------------------------------------------------------------

//     stdout.flush()?;
//     Ok(println!())
//     // unimplemented!("Open: Is not yet implemented!");
// }

pub async fn handle(noun: Noun) -> Result<()> {
    println!("Open: Reading configuration file");
    let (config, store) = match config::read_config_toml() {
        Ok(config) => {
            let store = WnfsStore::new(
                Url::parse(&config.store_url)?,
                None,
                wnfs_store::DataStoreKind::Memory,
            );
            (config, store)
        }
        Err(err) => {
            println!("Open: Error reading configuration file: {}", err);
            println!("Open: Creating default configuration file");
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

    todo!("Open: Is not yet completely implemented!")
}
