pub const CHUNK_SIZE: usize = 16;
enum BlockId {
    Air = 0,
    Bedrock = 1,
    Dirt = 2,
    Grass = 3,
}
pub fn generate_superflat_chunk_packet(chunk_x: i32, chunk_z: i32, height: u16) -> Vec<u8> {
    let bytes_per_block: u8 = 1;
    let voxel_count = (CHUNK_SIZE * CHUNK_SIZE) as u32 * height as u32;

    let mut payload = vec![BlockId::Air as u8; voxel_count as usize];

    for y in 0..height as usize {
        let id = if y == 0 {
            BlockId::Bedrock
        } else if (1..=2).contains(&y) {
            BlockId::Dirt
        } else if y == 3 {
            BlockId::Grass
        } else {
            BlockId::Air
        } as u8;

        for z in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                let idx = linear_index(x, y, z);
                payload[idx] = id;
            }
        }
    }

    let payload_len = payload.len() as u32;

    let mut out = Vec::with_capacity(4 + 4 + 2 + 1 + 4 + payload.len());
    out.extend_from_slice(&chunk_x.to_le_bytes());
    out.extend_from_slice(&chunk_z.to_le_bytes());
    out.extend_from_slice(&height.to_le_bytes());
    out.push(bytes_per_block);
    out.extend_from_slice(&payload_len.to_le_bytes());
    out.extend_from_slice(&payload);
    out
}
fn linear_index(x: usize, y: usize, z: usize) -> usize {
    y * CHUNK_SIZE * CHUNK_SIZE + z * CHUNK_SIZE + x
}
pub fn generate_view_radius_packets(
    center_cx: i32,
    center_cz: i32,
    radius: i32,
    height: u16,
) -> Vec<Vec<u8>> {
    let mut out = Vec::new();
    for dz in -radius..=radius {
        for dx in -radius..=radius {
            let cx = center_cx + dx;
            let cz = center_cz + dz;
            out.push(generate_superflat_chunk_packet(cx, cz, height));
        }
    }
    out
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn payload_size_ok() {
        let h = 64u16;
        let pkt = generate_superflat_chunk_packet(0, 0, h);
        let header = 15usize;
        let payload = pkt.len() - header;
        assert_eq!(payload, 16 * 16 * h as usize);
    }

    #[test]
    fn layers_ok() {
        let h = 8u16;
        let pkt = generate_superflat_chunk_packet(0, 0, h);
        let header = 15usize;
        let data = &pkt[header..];

        // y=0 → bedrock
        for z in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                let idx = z * CHUNK_SIZE + x;
                assert_eq!(data[idx], BlockId::Bedrock as u8);
            }
        }

        // y=1..=2 → dirt
        for y in 1..=2 {
            for z in 0..CHUNK_SIZE {
                for x in 0..CHUNK_SIZE {
                    let idx = y * CHUNK_SIZE * CHUNK_SIZE + z * CHUNK_SIZE + x;
                    assert_eq!(data[idx], BlockId::Dirt as u8);
                }
            }
        }

        // y=3 → grass
        for z in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                let idx = 3 * CHUNK_SIZE * CHUNK_SIZE + z * CHUNK_SIZE + x;
                assert_eq!(data[idx], BlockId::Grass as u8);
            }
        }

        // y>=4 → air
        for y in 4..h as usize {
            for z in 0..CHUNK_SIZE {
                for x in 0..CHUNK_SIZE {
                    let idx = y * CHUNK_SIZE * CHUNK_SIZE + z * CHUNK_SIZE + x;
                    assert_eq!(data[idx], BlockId::Air as u8);
                }
            }
        }
    }
}
