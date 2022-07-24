use crate::domains::{
    association::{Association, AssociationRepository},
    value::association::AssociationId,
};
use anyhow::Result;

pub fn get_association_list(repository: impl AssociationRepository) -> Result<Vec<Association>> {
    repository.find_all()
}

pub fn get_association_by_id(
    repository: impl AssociationRepository,
    association_id: AssociationId,
) -> Result<Association> {
    repository.find_by_id(association_id)
}

pub fn search_association_by_name(
    repository: impl AssociationRepository,
    name: String,
) -> Result<Vec<Association>> {
    repository.search_by_name(name)
}

pub fn post_association(
    repository: impl AssociationRepository,
    association: &Association,
) -> Result<Association> {
    repository.insert(association)
}

pub fn put_association(
    repository: impl AssociationRepository,
    association: &Association,
) -> Result<Association> {
    repository.update(association)
}

pub fn delete_association(
    repository: impl AssociationRepository,
    association_id: AssociationId,
) -> Result<Association> {
    repository.delete(association_id)
}
