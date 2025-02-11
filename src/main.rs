// Example Code

use noise::{NoiseFn, Perlin};
#[derive(Debug, Clone, Copy)]
enum BlockType {
    Air,
    Grass,
    Dirt,
    Stone,
    Water,
}
const CHUNK_SIZE: usize = 16;
const CHUNK_HEIGHT: usize = 128;

type Chunk = Vec<Vec<Vec<BlockType>>>;

fn main() {
    let perlin = Perlin::new(42);
    let scale = 0.1;
    let x = 10.0 * scale;
    let y = 20.0 * scale;
    let noise_value = perlin.get([x, y]);
    println!("Noise value: {}", noise_value);
}
fn fbm(perlin: &Perlin, x: f64, y: f64, octaves: u32, persistence: f64) -> f64 {
    let mut total = 0.0;
    let mut amplitude = 1.0;
    let mut frequency = 1.0;
    let mut max_value = 0.0;

    for _ in 0..octaves {
        total += perlin.get([x * frequency, y * frequency]) * amplitude;
        max_value += amplitude;
        amplitude *= persistence;
        frequency *= 2.0;
    }
    total / max_value
}

fn map_noise_to_height(noise_value: f64) -> usize {
    let normalized = (noise_value + 1.0) / 2.0; // map from [-1, 1] to [0, 1]
    let max_height = 128;
    (normalized * max_height as f64) as usize
}
fn generate_chunk(perlin: &Perlin, chunk_x: i32, chunk_z: i32) -> Chunk {
    let mut chunk = vec![vec![vec![BlockType::Air; CHUNK_HEIGHT]; CHUNK_SIZE]; CHUNK_SIZE];
    let scale = 0.1;

    for local_x in 0..CHUNK_SIZE {
        for local_z in 0..CHUNK_SIZE {
            let world_x = chunk_x * CHUNK_SIZE as i32 + local_x as i32;
            let world_z = chunk_z * CHUNK_SIZE as i32 + local_z as i32;
            let noise_value = fbm(
                perlin,
                world_x as f64 * scale,
                world_z as f64 * scale,
                4,
                0.5,
            );
            let height = map_noise_to_height(noise_value);

            for y in 0..CHUNK_HEIGHT {
                if y > height {
                    chunk[local_x][local_z][y] = if y < 64 {
                        BlockType::Water
                    } else {
                        BlockType::Air
                    };
                } else if y == height {
                    chunk[local_x][local_z][y] = BlockType::Grass;
                } else if y > height.saturating_sub(3) {
                    chunk[local_x][local_z][y] = BlockType::Dirt;
                } else {
                    chunk[local_x][local_z][y] = BlockType::Stone;
                }
            }
        }
    }
    chunk
}
