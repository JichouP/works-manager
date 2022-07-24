use crate::domains::{
    config::{Config, ConfigRepository},
    value::config::ConfigId,
};
use anyhow::Result;

fn get_config_list(repository: impl ConfigRepository) -> Result<Vec<Config>> {
    repository.find_all()
}

fn get_config_by_id(repository: impl ConfigRepository, config_id: ConfigId) -> Result<Config> {
    repository.find_by_id(config_id)
}

fn post_config(repository: impl ConfigRepository, config: &Config) -> Result<Config> {
    repository.insert(config)
}

fn put_config(repository: impl ConfigRepository, config: &Config) -> Result<Config> {
    repository.update(config)
}

fn delete_config(repository: impl ConfigRepository, config_id: ConfigId) -> Result<Config> {
    repository.delete(config_id)
}

pub fn get_config(repository: impl ConfigRepository) -> Result<Config> {
    repository.find_by_id(1)
}

pub fn update_config(repository: impl ConfigRepository, config: &Config) -> Result<Config> {
    let length = repository.find_all()?.len();

    match length {
        0 => repository.insert(config),
        _ => {
            let config = Config {
                id: 1,
                ..config.clone()
            };
            repository.update(&config)
        }
    }
}
