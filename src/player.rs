use nbt::{Tag, write_nbt};
use std::fs::File;
pub struct PlayerData {
    pub inventory: Vec<Item>,
    pub motion: [f64; 2],
    pub position: [f64; 3],
    pub rotation: [f32; 2],
    pub absorbtion_amount: f32,
    pub air: i16,
    pub current_impulse_context_reset_grace_time: i32,
    pub data_version: i32,
    pub death_time: i16,
    pub dimension: String,
    pub fall_distance: f64,
    pub fall_flying: bool,
    pub fire: i16,
    pub food_exhaustion_level: f32,
    pub food_level: i32,
    pub food_saturation_level: f32,
    pub food_tick_timer: i32,
    pub health: f32,
    pub hurt_by_timestamp: i32,
    pub hurt_time: i16,
    pub ignore_fall_damage_from_current_explosion: bool,
    pub invulnerable: bool,
    pub on_ground: bool,
    pub player_game_type: i32,
    pub portal_cooldown: i32,
    pub score: i32,
    pub seen_credits: bool,
    pub selected_item_slot: i32,
    pub sleep_timer: i16,
    pub spawn_extra_particles_on_fall: bool,
    pub xp_level: i32,
    pub xp_p: f32,
    pub xp_seed: i32,
    pub xp_total: i32,
    pub uuid: [i32; 4],
}
pub struct Item {
    pub count: i8,
    pub slot: i8,
    pub metadata: i16,
    pub id: String,
}
pub fn create_nbt(uuid: &String, player_data: PlayerData, path: String) -> std::io::Result<()> {
    let mut root = Tag::new_compound(uuid);
    let mut inventory = Tag::new_compound("inventory");
    let mut position = Tag::new_compound("Position");
    let mut motion = Tag::new_compound("Motion");
    for i in player_data.inventory {
        inventory.insert("Count".into(), Tag::new_byte("Count", i.count));
        inventory.insert("slot".into(), Tag::new_byte("slot", i.slot));
        inventory.insert("metadata".into(), Tag::new_short("metadata", i.metadata));
        inventory.insert("id".into(), Tag::new_string("id", i.id));
    }
    for i in player_data.motion {
        motion.insert("".into(), Tag::new_double("", i));
    }
    for i in player_data.position {
        position.insert("".into(), Tag::new_double("", i));
    }
    root.insert("inventory".into(), inventory);
    root.insert("motion".into(), motion);
    root.insert("position".into(), position);
    root.insert(
        "absorbtionAmount".into(),
        Tag::new_float("absorbtionAmount", player_data.absorbtion_amount),
    );
    root.insert("Air".into(), Tag::new_short("Air", player_data.air));
    root.insert(
        "current_impulse_context_reset_grace_time".into(),
        Tag::new_int(
            "current_impulse_context_reset_grace_time",
            player_data.current_impulse_context_reset_grace_time,
        ),
    );
    root.insert(
        "DataVersion".into(),
        Tag::new_int("DataVersion", player_data.data_version),
    );
    root.insert(
        "DeathTime".into(),
        Tag::new_short("DeathTime", player_data.death_time),
    );
    root.insert(
        "Dimension".into(),
        Tag::new_string("Dimension", player_data.dimension),
    );
    root.insert(
        "fall_distance".into(),
        Tag::new_double("fall_distance", player_data.fall_distance),
    );
    root.insert(
        "FallFlying".into(),
        Tag::new_byte("FallFlying", i8::from(player_data.fall_flying)),
    );
    root.insert("Fire".into(), Tag::new_short("Fire", player_data.fire));
    root.insert(
        "foodExhaustionLevel".into(),
        Tag::new_float("foodExhaustionLevel", player_data.food_exhaustion_level),
    );
    root.insert(
        "foodLevel".into(),
        Tag::new_int("foodLevel", player_data.food_level),
    );
    root.insert(
        "foodSaturationLevel".into(),
        Tag::new_float("foodSaturationLevel", player_data.food_saturation_level),
    );
    root.insert(
        "foodTickTimer".into(),
        Tag::new_int("foodTickTimer", player_data.food_tick_timer),
    );
    root.insert(
        "Health".into(),
        Tag::new_float("Health", player_data.health),
    );
    root.insert(
        "HurtByTimestamp".into(),
        Tag::new_int("HurtByTimestamp", player_data.hurt_by_timestamp),
    );
    root.insert(
        "HurtTime".into(),
        Tag::new_short("HurtTime", player_data.hurt_time),
    );
    root.insert(
        "ignore_fall_damage_from_current_explosion".into(),
        Tag::new_byte(
            "ignore_fall_damage_from_current_explosion",
            i8::from(player_data.ignore_fall_damage_from_current_explosion),
        ),
    );
    root.insert(
        "Invulnerable".into(),
        Tag::new_byte("Invulnerable", i8::from(player_data.invulnerable)),
    );
    root.insert(
        "OnGround".into(),
        Tag::new_byte("OnGround", i8::from(player_data.on_ground)),
    );
    root.insert(
        "playerGameType".into(),
        Tag::new_int("playerGameType", player_data.player_game_type),
    );
    root.insert(
        "PortalCooldown".into(),
        Tag::new_int("PortalCooldown", player_data.portal_cooldown),
    );
    root.insert("Score".into(), Tag::new_int("Score", player_data.score));
    root.insert(
        "seenCredits".into(),
        Tag::new_byte("seenCredits", i8::from(player_data.seen_credits)),
    );
    root.insert(
        "SelectedItemSlot".into(),
        Tag::new_int("SelectedItemSlot", player_data.selected_item_slot),
    );
    root.insert(
        "SleepTimer".into(),
        Tag::new_short("SleepTimer", player_data.sleep_timer),
    );
    root.insert(
        "spawn_extra_particles_on_fall".into(),
        Tag::new_byte(
            "spawn_extra_particles_on_fall",
            i8::from(player_data.spawn_extra_particles_on_fall),
        ),
    );
    root.insert(
        "XpLevel".into(),
        Tag::new_int("XpLevel", player_data.xp_level),
    );
    root.insert("XpP".into(), Tag::new_float("XpP", player_data.xp_p));
    root.insert("XpSeed".into(), Tag::new_int("XpSeed", player_data.xp_seed));
    root.insert(
        "XpTotal".into(),
        Tag::new_int("XpTotal", player_data.xp_total),
    );
    root.insert(
        "UUID".into(),
        Tag::new_int_array("UUID", player_data.uuid.into()),
    );

    let file = File::create(format!("{path}/{uuid}.dat"))?;
    write_nbt(&root, file)?;
    Ok(())
}
