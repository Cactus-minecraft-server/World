use std::fs::File;

use nbt::{Tag, write_nbt};

pub fn create_nbt(
    seed: i64,
    is_hardcore: bool,
    structures: bool,
    raining: bool,
    thundering: bool,
    game_type: i32,
    generator_version: i32,
    raintime: i32,
    spawnx: i32,
    spawny: i32,
    spawnz: i32,
    thundertime: i32,
    version: i32,
    last_played: i64,
    disk_size: i64,
    time: i64,
    generator_name: String,
    level_name: String,
    path: String,
) -> std::io::Result<()> {
    let mut root = Tag::new_compound("Data");
    root.insert("RandomSeed".to_string(), Tag::new_long("RandomSeed", seed));
    root.insert(
        "hardcore".to_string(),
        Tag::new_byte("hardcore", i8::from(is_hardcore)),
    );
    root.insert(
        "MapFeatures".to_string(),
        Tag::new_byte("MapFeatures", i8::from(structures)),
    );
    root.insert(
        "raining".to_string(),
        Tag::new_byte("raining", i8::from(raining)),
    );
    root.insert(
        "thundering".to_string(),
        Tag::new_byte("thundering", i8::from(thundering)),
    );
    root.insert("GameType".to_string(), Tag::new_int("GameType", game_type));
    root.insert(
        "GeneratorVersion".to_string(),
        Tag::new_int("GeneratorVersion", generator_version),
    );
    root.insert("RainTime".to_string(), Tag::new_int("RainTime", raintime));
    root.insert("SpawnX".to_string(), Tag::new_int("SpawnX", spawnx));
    root.insert("SpawnY".to_string(), Tag::new_int("SpawnY", spawny));
    root.insert("SpawnZ".to_string(), Tag::new_int("SpawnZ", spawnz));
    root.insert(
        "ThunderTime".to_string(),
        Tag::new_int("ThunderTime", thundertime),
    );
    root.insert("Version".to_string(), Tag::new_int("Version", version));
    root.insert(
        "LastPlayed".to_string(),
        Tag::new_long("LastPlayed", last_played),
    ); // Not sure if I should keep LastPlayed field because of the fact that this isn't for a
    // client world but for a server world
    root.insert(
        "SizeOnDisk".to_string(),
        Tag::new_long("SizeOnDisk", disk_size),
    );
    root.insert("Time".to_string(), Tag::new_long("Time", time));
    root.insert(
        "GeneratorName".to_string(),
        Tag::new_string("GeneratorName", generator_name),
    );
    root.insert(
        "LevelName".to_string(),
        Tag::new_string("LevelName", level_name),
    );
    let file = File::create(path)?;
    write_nbt(&root, file)?;
    Ok(())
}
