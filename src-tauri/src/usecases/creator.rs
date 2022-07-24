use crate::domains::{
    creator::{Creator, CreatorRepository, JoinedCreator},
    value::creator::CreatorId,
};
use anyhow::Result;

pub fn get_creator_list(repository: impl CreatorRepository) -> Result<Vec<JoinedCreator>> {
    repository.find_all()
}

pub fn get_creator_by_id(
    repository: impl CreatorRepository,
    creator_id: CreatorId,
) -> Result<Creator> {
    repository.find_by_id(creator_id)
}

pub fn search_creator_by_name(
    repository: impl CreatorRepository,
    name: String,
) -> Result<Vec<Creator>> {
    repository.search_by_name(name)
}

pub fn post_creator(repository: impl CreatorRepository, creator: &Creator) -> Result<Creator> {
    repository.insert(creator)
}

pub fn put_creator(repository: impl CreatorRepository, creator: &Creator) -> Result<Creator> {
    repository.update(creator)
}

pub fn delete_creator(
    repository: impl CreatorRepository,
    creator_id: CreatorId,
) -> Result<Creator> {
    repository.delete(creator_id)
}
