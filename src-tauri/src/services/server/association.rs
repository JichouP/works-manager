use anyhow::{Error, Result};
use rocket::{fairing::AdHoc, response::Debug, serde::json::Json, State};
use serde::Deserialize;

use crate::{
    domains::{association::Association, value::association::AssociationId},
    services::context::Context,
    usecases,
};

#[derive(Deserialize)]
struct NewAssociation {
    name: String,
}

#[get("/?<name>")]
fn get_association_list(
    ctx: &State<Context>,
    name: Option<String>,
) -> Result<Json<Vec<Association>>, Debug<Error>> {
    match name {
        Some(name) => {
            let result = usecases::association::search_association_by_name(
                ctx.association_repository(),
                name,
            )?;

            Ok(Json(result))
        }
        None => {
            let result = usecases::association::get_association_list(ctx.association_repository())?;

            Ok(Json(result))
        }
    }
}

#[post("/", format = "json", data = "<association>")]
fn post_association(
    ctx: &State<Context>,
    association: Json<NewAssociation>,
) -> Result<Json<Association>, Debug<Error>> {
    let association = Association::from(association.0.name);
    let result =
        usecases::association::post_association(ctx.association_repository(), &association)?;

    Ok(Json(result))
}

#[get("/<id>")]
fn get_association(
    ctx: &State<Context>,
    id: AssociationId,
) -> Result<Option<Json<Association>>, Debug<Error>> {
    let result = usecases::association::get_association_by_id(ctx.association_repository(), id);

    match result {
        Ok(association) => Ok(Some(Json(association))),
        Err(_) => Ok(None),
    }
}

#[put("/<id>", format = "json", data = "<association>")]
fn put_association(
    ctx: &State<Context>,
    id: AssociationId,
    association: Json<NewAssociation>,
) -> Result<Option<Json<Association>>, Debug<Error>> {
    let association = Association::new(id, association.0.name);
    let result = usecases::association::put_association(ctx.association_repository(), &association);

    match result {
        Ok(association) => Ok(Some(Json(association))),
        Err(_) => Ok(None),
    }
}

#[delete("/<id>")]
fn delete_association(
    ctx: &State<Context>,
    id: AssociationId,
) -> Result<Option<Json<Association>>, Debug<Error>> {
    let result = usecases::association::delete_association(ctx.association_repository(), id);

    match result {
        Ok(association) => Ok(Some(Json(association))),
        Err(_) => Ok(None),
    }
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Association", |rocket| async {
        rocket.mount(
            "/association",
            routes![
                get_association_list,
                post_association,
                get_association,
                put_association,
                delete_association,
            ],
        )
    })
}
