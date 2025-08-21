use std::io;

use crate::level::{LevelDat, create_nbt};

mod level;
mod perlin;
mod player;
mod superflat;
#[cfg(test)]
mod test;

pub fn create_level(path: &str, data: &LevelDat) -> Result<(), io::Error> {
    create_nbt(data, path)
}
