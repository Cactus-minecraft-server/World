/// Test for perlin.rs
#[cfg(test)]
mod perlin_test {
    use crate::perlin::{CHUNK_SIZE, MAX_Y, MIN_Y, generate_height_chunk};

    use image::{ImageBuffer, Luma};

    #[test]
    #[ignore]
    fn dump_chunk_png() {
        let seed: u64 = 42;
        let chunks_x: usize = 32;
        let chunks_z: usize = 32;

        let w: u32 = (chunks_x * CHUNK_SIZE) as u32;
        let h: u32 = (chunks_z * CHUNK_SIZE) as u32;

        let mut field = vec![0i32; (w as usize) * (h as usize)];

        for cz in 0..chunks_z {
            for cx in 0..chunks_x {
                let tile = generate_height_chunk(seed, cx as i32, cz as i32);
                for lz in 0..CHUNK_SIZE {
                    for lx in 0..CHUNK_SIZE {
                        let x = cx * CHUNK_SIZE + lx;
                        let z = cz * CHUNK_SIZE + lz;
                        field[z * (w as usize) + x] = tile[lx][lz];
                    }
                }
            }
        }

        let mut img: ImageBuffer<Luma<u16>, Vec<u16>> = ImageBuffer::new(w, h);
        let denom = (MAX_Y - MIN_Y) as f32;
        for z in 0..h {
            for x in 0..w {
                let v = field[(z as usize) * (w as usize) + (x as usize)];
                let n01 = ((v - MIN_Y) as f32 / denom).clamp(0.0, 1.0);
                let p = (n01 * u16::MAX as f32).round() as u16;
                img.put_pixel(x, z, Luma([p]));
            }
        }

        std::fs::create_dir_all("target").ok();
        img.save("target/heightmap16.png").unwrap();
    }
}
/// Test for level.rs
#[cfg(test)]
mod level_file_test {
    use crate::level::{
        CustomBossEvents, DataPacks, Dimension, DragonFight, LevelDat, ScheduledEvents,
        ServerBrands, VersionInfo, WorldGenSettings, create_nbt,
    };
    use std::collections::HashMap;

    #[test]
    fn test_creation_of_file() {
        let mut game_rules: HashMap<String, String> = HashMap::new();
        game_rules.insert("doDaylightCycle".into(), "true".into());

        let mut dimensions: HashMap<String, Dimension> = HashMap::new();
        dimensions.insert("minecraft:overworld".into(), Dimension::Overworld);

        let level = LevelDat {
            custom_boss_events: CustomBossEvents {},
            data_packs: DataPacks {
                disabled: vec![],
                enabled: vec![],
            },
            dragon_fight: DragonFight {
                gateways: vec![0, 1, 2],
                dragon_killed: false,
                needs_state_scanning: false,
                previously_killed: false,
            },
            game_rules,
            version: VersionInfo {
                id: 3465,
                name: "1.20.1".into(),
                series: "main".into(),
                snapshot: 0,
            },
            world_gen_settings: WorldGenSettings {
                dimensions,
                bonus_chest: false,
                generate_features: true,
                seed: 1234,
            },
            scheduled_events: ScheduledEvents {},
            server_brands: ServerBrands::from(vec!["vanilla".to_string()]),

            allow_commands: true,

            border_center_x: 0.0,
            border_center_z: 0.0,
            border_damage_per_block: 0.0,
            border_safe_zone: 0.0,
            border_size: 60_000_000.0,
            border_size_lerp_target: 60_000_000.0,
            border_size_lerp_time: 0,
            border_warning_blocks: 5,
            border_warning_time: 15,

            clear_weather_time: 0,
            data_version: 3465,
            day_time: 0,
            difficulty: 2,
            difficulty_locked: false,
            game_type: 0,
            hardcore: false,
            initialized: true,
            last_played: 0,
            level_name: "test".into(),
            raining: false,
            rain_time: 0,
            spawn_angle: 0.0,
            spawn_x: 0,
            spawn_y: 64,
            spawn_z: 0,
            thundering: false,
            thunder_time: 0,
            time: 0,
            version_id: 3465,
            wandering_trader_spawn_chance: 25,
            wandering_trader_spawn_delay: 1200,
            was_modded: false,
        };

        let result = create_nbt(&level, "target");
        assert!(result.is_ok());
    }
}

/// Test for player.rs
#[cfg(test)]
mod player_data_test {
    use crate::player::{Item, PlayerData, create_nbt};

    #[test]
    fn test_playerdata_creation() {
        let item = Item {
            count: 1,
            slot: 0,
            metadata: 0,
            id: "minecraft:stone".to_string(),
        };

        let data = PlayerData {
            inventory: vec![item],
            motion: [0.0, 0.0],
            position: [0.0, 64.0, 0.0],
            rotation: [0.0, 0.0],
            absorbtion_amount: 0.0,
            air: 300,
            current_impulse_context_reset_grace_time: 0,
            data_version: 3465,
            death_time: 0,
            dimension: "minecraft:overworld".to_string(),
            fall_distance: 0.0,
            fall_flying: false,
            fire: -20,
            food_exhaustion_level: 0.0,
            food_level: 20,
            food_saturation_level: 5.0,
            food_tick_timer: 0,
            health: 20.0,
            hurt_by_timestamp: 0,
            hurt_time: 0,
            ignore_fall_damage_from_current_explosion: false,
            invulnerable: false,
            on_ground: true,
            player_game_type: 0,
            portal_cooldown: 0,
            score: 0,
            seen_credits: false,
            selected_item_slot: 0,
            sleep_timer: 0,
            spawn_extra_particles_on_fall: false,
            xp_level: 0,
            xp_p: 0.0,
            xp_seed: 0,
            xp_total: 0,
            uuid: [0, 0, 0, 0],
        };
        let result = create_nbt(
            &"8c701aa5-e353-42dd-aa71-95d76b63a5d7".into(),
            data,
            "target/".into(),
        );

        assert_eq!(result.is_ok(), true);
    }
}
