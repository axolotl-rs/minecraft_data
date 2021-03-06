use crate::GenResult;
use minecraft_data_rs::models::enchantment::Enchantment;
use std::fs::write;
use std::path::PathBuf;

use crate::error::GenError;

pub type Root = Vec<Enchantment>;

pub fn generate_enchantments(file: PathBuf, _json: Root) -> GenResult<()> {
    let string = String::new();

    write(file, string).map_err(GenError::Io)
}
