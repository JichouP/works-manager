#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![allow(dead_code)]

use anyhow::Result;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate rocket;

mod domains;
mod infrastructures;
mod services;
mod usecases;

#[cfg(test)]
mod tests;

embed_migrations!("migrations");

#[rocket::main]
async fn main() -> Result<()> {
    let ctx = services::context::Context::new("db.sqlite");
    embedded_migrations::run(&ctx.pool.get()?)?;

    rocket::tokio::spawn(async move {
        services::server::run(ctx)
            .await
            .expect("error while running server")
    });

    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
