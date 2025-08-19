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
    use crate::level::create_nbt;
    #[test]
    fn test_creation_of_file() -> () {
        let result = create_nbt(
            "test".to_string(),
            1234,
            true,
            true,
            true,
            true,
            1,
            1,
            1,
            0,
            0,
            0,
            213,
            1235,
            55555,
            90000,
            900,
            "test".to_string(),
            "test".to_string(),
            "target/level.dat".to_string(),
        );
        assert_eq!(result.is_ok(), true);
    }
}
