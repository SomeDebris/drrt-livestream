use std::env;

use anyhow::Result;
use base64::engine::{general_purpose, Engine};
use obws::{Client};
use tokio::fs;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    env::set_var("RUST_LOG", "obws=debug");
    tracing_subscriber::fmt::init();

    let client = Client::connect("localhost", 4455, env::var("OBS_PASSWORD").ok()).await?;

    // Get and print out version information of OBS and obs-websocket.
    let version = client.general().version().await?;
    println!("{:#?}", version);

    // Get a list of available scenes and print them out.
    let scene_list = client.scenes().list().await?;
    println!("{:#?}", scene_list);

    let hotkey_list = client.hotkeys().list().await?;
    println!("{:#?}", hotkey_list);

    let dsk_thing = client
        .hotkeys()
        .trigger_by_name("Hide on DSK 1")
        .await?;

    Ok(())
}
