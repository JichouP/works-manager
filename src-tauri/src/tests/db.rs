use super::util::init;
use crate::domains::creator::JoinedCreator;
use crate::{measure, services};

#[test]
fn test_get_all() -> anyhow::Result<()> {
    let ctx = init()?;
    let base: usize = 1000;

    for i in 0..10 {
        insert_many(&ctx, base)?;
        measure!(
            format!("get_all_{:<6}", (i + 1) * base),
            get_all(&ctx, (i + 1) * base)?
        );
    }

    Ok(())
}

fn insert_many(ctx: &services::context::Context, times: usize) -> anyhow::Result<()> {
    use crate::domains::creator::Creator;
    use crate::usecases;

    for i in 0..times {
        usecases::creator::post_creator(
            ctx.creator_repository(),
            &Creator::from(format!("test creator {}", i), None),
        )?;
    }

    Ok(())
}

fn get_all(ctx: &services::context::Context, times: usize) -> anyhow::Result<Vec<JoinedCreator>> {
    use crate::usecases;

    let result = usecases::creator::get_creator_list(ctx.creator_repository())?;

    assert_eq!(result.len(), times);

    Ok(result)
}
