use crate::services;

#[macro_export]
macro_rules! measure {
    ($n: expr, $x: expr) => {{
        use std::time::Instant;

        let start = Instant::now();
        let result = $x;
        let end = start.elapsed();
        println!("{}: {}.{:03}", $n, end.as_secs(), end.subsec_millis());

        result
    }};
}

pub fn init() -> anyhow::Result<services::context::Context> {
    use crate::embedded_migrations;

    let ctx = services::context::Context::new(":memory:");

    embedded_migrations::run(&ctx.pool.get()?)?;

    Ok(ctx)
}

pub fn init_with_db(db_path: &str) -> anyhow::Result<services::context::Context> {
    use crate::embedded_migrations;

    let ctx = services::context::Context::new(db_path);

    embedded_migrations::run(&ctx.pool.get()?)?;

    Ok(ctx)
}
