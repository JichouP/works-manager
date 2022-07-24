use crate::tests::util::init;

#[test]
fn test() -> anyhow::Result<()> {
    use crate::domains::association::Association;
    use crate::{embedded_migrations, services, usecases};

    let ctx = services::context::Context::new(":memory:");
    embedded_migrations::run(&ctx.pool.get()?)?;
    let new_association = Association::from("test".to_string());
    usecases::association::post_association(ctx.association_repository(), &new_association)?;

    let result = usecases::association::get_association_list(ctx.association_repository())?;
    println!("{:?}", result[0].id.to_string());
    assert_eq!(1, 1);

    Ok(())
}

#[test]
fn creator() -> anyhow::Result<()> {
    use crate::domains::{association::Association, creator::Creator};
    use crate::usecases;

    let ctx = init()?;

    let new_association = usecases::association::post_association(
        ctx.association_repository(),
        &Association::from("test association".to_string()),
    )?;

    usecases::creator::post_creator(
        ctx.creator_repository(),
        &Creator::from("test creator".to_string(), Some(new_association.id)),
    )?;

    let result = usecases::creator::get_creator_list(ctx.creator_repository())?;

    println!("{:?}", result);

    Ok(())
}
