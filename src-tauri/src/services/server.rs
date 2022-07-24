use crate::services::context::Context;
use anyhow::Result;
use rocket::Config;

pub mod association;

const PORT: u16 = 12321;

pub async fn run(ctx: Context) -> Result<()> {
    rocket::build()
        .manage(ctx)
        .configure(Config {
            port: PORT,
            ..Config::default()
        })
        .attach(association::stage())
        .launch()
        .await?;

    Ok(())
}
