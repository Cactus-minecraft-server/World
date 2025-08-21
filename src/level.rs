use nbt::{Tag, write_nbt};
use std::collections::HashMap;
use std::fs::File;

pub struct LevelDat {
    pub custom_boss_events: CustomBossEvents,
    pub data_packs: DataPacks,
    pub dragon_fight: DragonFight,
    pub game_rules: GameRules,
    pub version: VersionInfo,
    pub world_gen_settings: WorldGenSettings,
    pub scheduled_events: ScheduledEvents,
    pub server_brands: ServerBrands,

    pub allow_commands: bool,

    pub border_center_x: f64,
    pub border_center_z: f64,
    pub border_damage_per_block: f64,
    pub border_safe_zone: f64,
    pub border_size: f64,
    pub border_size_lerp_target: f64,
    pub border_size_lerp_time: i64,
    pub border_warning_blocks: i32,
    pub border_warning_time: i32,

    pub clear_weather_time: i32,
    pub data_version: i32,
    pub day_time: i64,
    pub difficulty: i8,
    pub difficulty_locked: bool,
    pub game_type: i32,
    pub hardcore: bool,
    pub initialized: bool,
    pub last_played: i64,
    pub level_name: String,
    pub raining: bool,
    pub rain_time: i32,
    pub spawn_angle: f32,
    pub spawn_x: i32,
    pub spawn_y: i32,
    pub spawn_z: i32,
    pub thundering: bool,
    pub thunder_time: i32,
    pub time: i64,
    pub version_id: i32,
    pub wandering_trader_spawn_chance: i32,
    pub wandering_trader_spawn_delay: i32,
    pub was_modded: bool,
}

pub struct CustomBossEvents {}

pub struct DataPacks {
    pub disabled: Vec<String>,
    pub enabled: Vec<String>,
}

pub struct DragonFight {
    pub gateways: Vec<i32>,
    pub dragon_killed: bool,
    pub needs_state_scanning: bool,
    pub previously_killed: bool,
}

pub type GameRules = HashMap<String, String>;

pub struct VersionInfo {
    pub id: i32,
    pub name: String,
    pub series: String,
    pub snapshot: i8,
}

pub enum Dimension {
    Overworld,
    End,
    Nether,
}

pub struct WorldGenSettings {
    pub dimensions: HashMap<String, Dimension>,
    pub bonus_chest: bool,
    pub generate_features: bool,
    pub seed: i64,
}

pub struct ScheduledEvents {}

pub type ServerBrands = Vec<String>;
fn dim_to_str(d: &Dimension) -> &'static str {
    match d {
        Dimension::Overworld => "overworld",
        Dimension::End => "the_end",
        Dimension::Nether => "the_nether",
    }
}

pub fn create_nbt(level: &LevelDat, path: &str) -> std::io::Result<()> {
    let mut root = Tag::new_compound("Data");

    // --- primitives (root) ---
    root.insert(
        "allowCommands".to_string(),
        Tag::new_byte("allowCommands", i8::from(level.allow_commands)),
    );

    root.insert(
        "BorderCenterX".to_string(),
        Tag::new_double("BorderCenterX", level.border_center_x),
    );
    root.insert(
        "BorderCenterZ".to_string(),
        Tag::new_double("BorderCenterZ", level.border_center_z),
    );
    root.insert(
        "BorderDamagePerBlock".to_string(),
        Tag::new_double("BorderDamagePerBlock", level.border_damage_per_block),
    );
    root.insert(
        "BorderSafeZone".to_string(),
        Tag::new_double("BorderSafeZone", level.border_safe_zone),
    );
    root.insert(
        "BorderSize".to_string(),
        Tag::new_double("BorderSize", level.border_size),
    );
    root.insert(
        "BorderSizeLerpTarget".to_string(),
        Tag::new_double("BorderSizeLerpTarget", level.border_size_lerp_target),
    );
    root.insert(
        "BorderSizeLerpTime".to_string(),
        Tag::new_long("BorderSizeLerpTime", level.border_size_lerp_time),
    );
    root.insert(
        "BorderWarningBlocks".to_string(),
        Tag::new_int("BorderWarningBlocks", level.border_warning_blocks),
    );
    root.insert(
        "BorderWarningTime".to_string(),
        Tag::new_int("BorderWarningTime", level.border_warning_time),
    );

    root.insert(
        "clearWeatherTime".to_string(),
        Tag::new_int("clearWeatherTime", level.clear_weather_time),
    );
    root.insert(
        "DataVersion".to_string(),
        Tag::new_int("DataVersion", level.data_version),
    );
    root.insert(
        "DayTime".to_string(),
        Tag::new_long("DayTime", level.day_time),
    );
    root.insert(
        "Difficulty".to_string(),
        Tag::new_byte("Difficulty", level.difficulty),
    );
    root.insert(
        "DifficultyLocked".to_string(),
        Tag::new_byte("DifficultyLocked", i8::from(level.difficulty_locked)),
    );
    root.insert(
        "GameType".to_string(),
        Tag::new_int("GameType", level.game_type),
    );
    root.insert(
        "hardcore".to_string(),
        Tag::new_byte("hardcore", i8::from(level.hardcore)),
    );
    root.insert(
        "initialized".to_string(),
        Tag::new_byte("initialized", i8::from(level.initialized)),
    );
    root.insert(
        "LastPlayed".to_string(),
        Tag::new_long("LastPlayed", level.last_played),
    );
    root.insert(
        "LevelName".to_string(),
        Tag::new_string("LevelName", level.level_name.clone()),
    );
    root.insert(
        "raining".to_string(),
        Tag::new_byte("raining", i8::from(level.raining)),
    );
    root.insert(
        "RainTime".to_string(),
        Tag::new_int("RainTime", level.rain_time),
    );
    root.insert(
        "SpawnAngle".to_string(),
        Tag::new_float("SpawnAngle", level.spawn_angle),
    );
    root.insert("SpawnX".to_string(), Tag::new_int("SpawnX", level.spawn_x));
    root.insert("SpawnY".to_string(), Tag::new_int("SpawnY", level.spawn_y));
    root.insert("SpawnZ".to_string(), Tag::new_int("SpawnZ", level.spawn_z));
    root.insert(
        "thundering".to_string(),
        Tag::new_byte("thundering", i8::from(level.thundering)),
    );
    root.insert(
        "ThunderTime".to_string(),
        Tag::new_int("ThunderTime", level.thunder_time),
    );
    root.insert("Time".to_string(), Tag::new_long("Time", level.time));
    root.insert(
        "version".to_string(),
        Tag::new_int("version", level.version_id),
    );
    root.insert(
        "WanderingTraderSpawnChance".to_string(),
        Tag::new_int(
            "WanderingTraderSpawnChance",
            level.wandering_trader_spawn_chance,
        ),
    );
    root.insert(
        "WanderingTraderSpawnDelay".to_string(),
        Tag::new_int(
            "WanderingTraderSpawnDelay",
            level.wandering_trader_spawn_delay,
        ),
    );
    root.insert(
        "WasModded".to_string(),
        Tag::new_byte("WasModded", i8::from(level.was_modded)),
    );

    // Mirror of older key from your example, sourced from struct:
    root.insert(
        "RandomSeed".to_string(),
        Tag::new_long("RandomSeed", level.world_gen_settings.seed),
    );
    root.insert(
        "MapFeatures".to_string(),
        Tag::new_byte(
            "MapFeatures",
            i8::from(level.world_gen_settings.generate_features),
        ),
    );

    // --- GameRules ---
    let mut gr = Tag::new_compound("GameRules");
    for (k, v) in &level.game_rules {
        gr.insert(k.clone(), Tag::new_string(k, v.clone()));
    }
    root.insert("GameRules".to_string(), gr);

    // --- Version (compound) ---
    let mut ver = Tag::new_compound("Version");
    ver.insert("Id".to_string(), Tag::new_int("Id", level.version.id));
    ver.insert(
        "Name".to_string(),
        Tag::new_string("Name", level.version.name.clone()),
    );
    ver.insert(
        "Series".to_string(),
        Tag::new_string("Series", level.version.series.clone()),
    );
    ver.insert(
        "Snapshot".to_string(),
        Tag::new_byte("Snapshot", level.version.snapshot),
    );
    root.insert("Version".to_string(), ver);

    // --- DataPacks ---
    let mut dp = Tag::new_compound("DataPacks");
    let enabled_list: Vec<Tag> = level
        .data_packs
        .enabled
        .iter()
        .cloned()
        .map(|s| Tag::new_string("", s))
        .collect();
    let disabled_list: Vec<Tag> = level
        .data_packs
        .disabled
        .iter()
        .cloned()
        .map(|s| Tag::new_string("", s))
        .collect();
    dp.insert(
        "Enabled".to_string(),
        Tag::new_list("Enabled", 8, enabled_list),
    );
    dp.insert(
        "Disabled".to_string(),
        Tag::new_list("Disabled", 8, disabled_list),
    );

    root.insert("DataPacks".to_string(), dp);

    // --- DragonFight ---
    let mut df = Tag::new_compound("DragonFight");
    let gateways_list: Vec<Tag> = level
        .dragon_fight
        .gateways
        .iter()
        .copied()
        .map(|n| Tag::new_int("", n))
        .collect();
    df.insert(
        "Gateways".to_string(),
        Tag::new_list("Gateways", 3, gateways_list),
    );
    df.insert(
        "DragonKilled".to_string(),
        Tag::new_byte("DragonKilled", i8::from(level.dragon_fight.dragon_killed)),
    );
    df.insert(
        "NeedsStateScanning".to_string(),
        Tag::new_byte(
            "NeedsStateScanning",
            i8::from(level.dragon_fight.needs_state_scanning),
        ),
    );
    df.insert(
        "PreviouslyKilled".to_string(),
        Tag::new_byte(
            "PreviouslyKilled",
            i8::from(level.dragon_fight.previously_killed),
        ),
    );
    root.insert("DragonFight".to_string(), df);

    // --- WorldGenSettings ---
    let mut dims = Tag::new_compound("dimensions");
    for (name, dim) in &level.world_gen_settings.dimensions {
        dims.insert(
            name.clone(),
            Tag::new_string(name, dim_to_str(dim).to_string()),
        );
    }
    let mut wgs = Tag::new_compound("WorldGenSettings");
    wgs.insert(
        "seed".to_string(),
        Tag::new_long("seed", level.world_gen_settings.seed),
    );
    wgs.insert(
        "generate_features".to_string(),
        Tag::new_byte(
            "generate_features",
            i8::from(level.world_gen_settings.generate_features),
        ),
    );
    wgs.insert(
        "bonus_chest".to_string(),
        Tag::new_byte(
            "bonus_chest",
            i8::from(level.world_gen_settings.bonus_chest),
        ),
    );
    wgs.insert("dimensions".to_string(), dims);
    root.insert("WorldGenSettings".to_string(), wgs);

    // --- ServerBrands ---
    let brands_list: Vec<Tag> = level
        .server_brands
        .iter()
        .cloned()
        .map(|s| Tag::new_string("", s))
        .collect();
    root.insert(
        "ServerBrands".to_string(),
        Tag::new_list("ServerBrands", 8, brands_list),
    );

    // --- CustomBossEvents / ScheduledEvents (empty compounds for now) ---
    root.insert(
        "CustomBossEvents".to_string(),
        Tag::new_compound("CustomBossEvents"),
    );
    root.insert(
        "ScheduledEvents".to_string(),
        Tag::new_compound("ScheduledEvents"),
    );

    // --- write ---
    let file = File::create(format!("{path}/level.dat"))?;
    write_nbt(&root, file)?;
    Ok(())
}
