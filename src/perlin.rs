use rand::{
    SeedableRng,
    distr::{Distribution, Uniform},
};
use rand_chacha::ChaCha8Rng;

pub const CHUNK_SIZE: usize = 16;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Noise {
    scale: f32,
    amplitude: f32,
    seed: u64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector {
    x: f32,
    y: f32,
}

pub const MIN_Y: i32 = -64;
pub const MAX_Y: i32 = 320;
pub const SEA_LEVEL: i32 = 63;

#[inline]
fn fbm_seeded(
    seed: u64,
    scale: f32,
    x: f32,
    z: f32,
    octaves: u32,
    persistence: f32,
    lacunarity: f32,
) -> f32 {
    let base = Noise::new(scale, 1.0, seed);
    let (mut amp, mut freq, mut sum, mut norm) = (1.0f32, 1.0f32, 0.0f32, 0.0f32);
    for _ in 0..octaves {
        sum += amp * base.get(x * freq, z * freq);
        norm += amp;
        amp *= persistence;
        freq *= lacunarity;
    }
    (sum / norm).clamp(-1.0, 1.0)
}

pub fn generate_height_chunk(seed: u64, cx: i32, cz: i32) -> [[i32; CHUNK_SIZE]; CHUNK_SIZE] {
    const SALT_CONT: u64 = 0xC0DEC0DEu64;
    const SALT_MNT: u64 = 0xBEEF1234u64;
    const SALT_DET: u64 = 0xDE7ACAFEu64;
    const SALT_ERO: u64 = 0xE20DFACEu64;

    const GAIN: f32 = 220.0;

    let mut out = [[SEA_LEVEL; CHUNK_SIZE]; CHUNK_SIZE];

    for lz in 0..CHUNK_SIZE {
        for lx in 0..CHUNK_SIZE {
            let xw = (cx * CHUNK_SIZE as i32 + lx as i32) as f32;
            let zw = (cz * CHUNK_SIZE as i32 + lz as i32) as f32;

            let cont = fbm_seeded(seed ^ SALT_CONT, 1500.0, xw, zw, 6, 0.5, 2.0); // [-1,1]
            let mnt = fbm_seeded(seed ^ SALT_MNT, 400.0, xw, zw, 6, 0.5, 2.0);
            let det = fbm_seeded(seed ^ SALT_DET, 48.0, xw, zw, 5, 0.5, 2.0);
            let eros = fbm_seeded(seed ^ SALT_ERO, 1200.0, xw, zw, 5, 0.5, 2.0);

            let ridge = (1.0 - mnt.abs()).powf(1.5); // [0,1]
            let c_cont = cont * 0.5; // [-0.5,0.5]
            let c_ridge = ridge - 0.5; // [-0.5,0.5]
            let c_det = det * 0.25; // [-0.25,0.25]
            let c_eros = eros * 0.35; // [-0.35,0.35]

            let s = 0.9 * c_cont + 0.8 * c_ridge + 0.25 * c_det - 0.35 * c_eros;

            let mut y = SEA_LEVEL as f32 + s * GAIN;
            y = y.clamp(MIN_Y as f32, MAX_Y as f32);
            out[lx][lz] = y.round() as i32;
        }
    }

    out
}
impl Noise {
    /// Construct a noise generator.
    /// `scale` must be non-zero (it rescales inputs onto the integer grid).
    pub fn new(scale: f32, amplitude: f32, seed: u64) -> Self {
        assert!(scale != 0.0, "scale must be non-zero");
        Self {
            scale,
            amplitude,
            seed,
        }
    }

    /// Query noise at world coordinates (x, z).
    /// Internally rescales by `self.scale` and returns an amplitude-scaled value.
    pub fn get(&self, x: f32, z: f32) -> f32 {
        let xs = x / self.scale;
        let zs = z / self.scale;
        self.perlin(xs, zs) * self.amplitude
    }

    /// Standard 2D Perlin noise:
    /// - Pick a unit gradient at each lattice corner via a hash.
    /// - Dot product with displacement vectors from corners to point.
    /// - Smoothstep (quintic) fade on each axis.
    /// - Bilinear interpolation using the fade weights.
    fn perlin(&self, x: f32, y: f32) -> f32 {
        // Integer lattice cell containing (x, y)
        let xi = x.floor() as i32;
        let yi = y.floor() as i32;

        // Local coordinates within the cell in [0, 1)
        let xf = x - xi as f32;
        let yf = y - yi as f32;

        // Corner gradients (deterministic from lattice coords + seed)
        let g00 = gradient_at(xi, yi, self.seed);
        let g10 = gradient_at(xi + 1, yi, self.seed);
        let g01 = gradient_at(xi, yi + 1, self.seed);
        let g11 = gradient_at(xi + 1, yi + 1, self.seed);

        // Displacement vectors from corners to (x, y)
        let v00 = Vector { x: xf, y: yf };
        let v10 = Vector { x: xf - 1.0, y: yf };
        let v01 = Vector { x: xf, y: yf - 1.0 };
        let v11 = Vector {
            x: xf - 1.0,
            y: yf - 1.0,
        };

        // Corner contributions (projection of displacement onto gradient)
        let n00 = dot_product(&g00, &v00);
        let n10 = dot_product(&g10, &v10);
        let n01 = dot_product(&g01, &v01);
        let n11 = dot_product(&g11, &v11);

        // Smooth weights along each axis
        let u = fade(xf);
        let v = fade(yf);

        // Interpolate along x on bottom and top edges, then along y
        let nx0 = linear_interpolation(n00, n10, u); // bottom edge
        let nx1 = linear_interpolation(n01, n11, u); // top edge
        linear_interpolation(nx0, nx1, v)
    }
}

/// 2D integer hash to 64-bit state, mixed with a global seed.
/// Produces well-distributed pseudorandom bits for gradient selection.
fn hash2(ix: i32, iy: i32, seed: u64) -> u64 {
    let mut h = seed
        ^ (ix as u64).wrapping_mul(0x9E3779B97F4A7C15)
        ^ (iy as u64).wrapping_mul(0xC2B2AE3D27D4EB4F);
    h ^= h >> 33;
    h = h.wrapping_mul(0xFF51AFD7ED558CCD);
    h ^= h >> 33;
    h = h.wrapping_mul(0xC4CEB9FE1A85EC53);
    h ^= h >> 33;
    h
}

/// Deterministic unit gradient at lattice point (ix, iy) from `seed`.
fn gradient_at(ix: i32, iy: i32, seed: u64) -> Vector {
    let h = hash2(ix, iy, seed);
    let mut rng = ChaCha8Rng::seed_from_u64(h);

    // [0, 2π]
    let dist = Uniform::new(0.0f32, std::f32::consts::TAU).unwrap();
    let angle: f32 = dist.sample(&mut rng);

    Vector {
        x: angle.cos(),
        y: angle.sin(),
    }
}

/// Dot product of two 2D vectors.
fn dot_product(v1: &Vector, v2: &Vector) -> f32 {
    v1.x * v2.x + v1.y * v2.y
}

/// Euclidean norm (length) of a 2D vector.
fn calculate_norm(v1: &Vector) -> f32 {
    v1.x.hypot(v1.y)
}

/// Safe normalization to unit length.
/// Returns the zero vector unchanged to avoid division by zero / NaNs.
fn normalize(v1: &Vector) -> Vector {
    let n = calculate_norm(v1);
    if n == 0.0 {
        return Vector { x: 0.0, y: 0.0 };
    }
    Vector {
        x: v1.x / n,
        y: v1.y / n,
    }
}

/// Quintic fade function used by Perlin noise.
/// Maps x ∈ [0,1] to a smooth S-curve with zero first derivative at 0 and 1.
/// 6x⁵ − 15x⁴ + 10x³
fn fade(x: f32) -> f32 {
    6_f32 * x.powi(5) - 15_f32 * x.powi(4) + 10_f32 * x.powi(3)
}

/// Linear interpolation between scalars `a` and `b` by factor `t`.
fn linear_interpolation(a: f32, b: f32, t: f32) -> f32 {
    a + t * (b - a)
}

#[cfg(test)]
mod tests {
    use crate::perlin::{
        Noise, Vector, calculate_norm, dot_product, fade, linear_interpolation, normalize,
    };

    fn deriv(f: fn(f32) -> f32, x: f32) -> f32 {
        let h = 1e-3;
        (f(x + h) - f(x - h)) / (2.0 * h)
    }

    fn approx_eq(a: f32, b: f32, eps: f32) -> bool {
        (a - b).abs() <= eps
    }

    #[test]
    fn test_dot_product() {
        assert!(approx_eq(
            dot_product(&Vector { x: 1.0, y: 0.0 }, &Vector { x: 0.0, y: 1.0 }),
            0.0,
            1e-6
        ));
        assert!(approx_eq(
            dot_product(&Vector { x: 1.0, y: 0.5 }, &Vector { x: 0.2, y: 1.0 }),
            0.7,
            1e-6
        ));
        assert!(approx_eq(
            dot_product(&Vector { x: 1.0, y: 0.5 }, &Vector { x: -0.2, y: -1.0 }),
            -0.7,
            1e-6
        ));
    }

    #[test]
    fn test_calculate_norm() {
        assert!(approx_eq(
            calculate_norm(&Vector { x: 0.5, y: 0.5 }),
            0.5_f32.sqrt(),
            1e-6
        ));
        assert!(approx_eq(
            calculate_norm(&Vector { x: 0.7, y: 0.3 }),
            0.58_f32.sqrt(),
            1e-6
        ));
        assert!(approx_eq(
            calculate_norm(&Vector { x: -0.7, y: -0.3 }),
            calculate_norm(&Vector { x: 0.7, y: 0.3 }),
            1e-6
        ));
    }

    #[test]
    fn test_normalize() {
        let v1 = Vector { x: 0.5, y: 0.5 };
        assert!(approx_eq(calculate_norm(&normalize(&v1)), 1.0, 1e-6));
        let z = Vector { x: 0.0, y: 0.0 };
        assert_eq!(normalize(&z), z);
    }

    #[test]
    fn test_linear_interpolation() {
        assert!(approx_eq(linear_interpolation(0.0, 10.0, 0.25), 2.5, 1e-6));
        assert!(approx_eq(linear_interpolation(5.0, 15.0, 0.75), 12.5, 1e-6));
    }

    #[test]
    fn dot_product_properties() {
        let a = Vector { x: 0.3, y: -0.7 };
        let b = Vector { x: -0.2, y: 1.1 };
        assert!((dot_product(&a, &b) - dot_product(&b, &a)).abs() < 1e-6);

        let k = 2.5;
        let kb = Vector {
            x: k * b.x,
            y: k * b.y,
        };
        assert!((dot_product(&a, &kb) - k * dot_product(&a, &b)).abs() < 1e-6);

        let p = Vector { x: 2.0, y: 4.0 };
        assert!(dot_product(&p, &p) >= 0.0);
    }

    #[test]
    fn norm_properties() {
        let a = Vector { x: -0.7, y: 0.3 };
        let b = Vector { x: 0.4, y: -1.2 };
        assert!(
            (calculate_norm(&a)
                - calculate_norm(&Vector {
                    x: a.x.abs(),
                    y: a.y.abs()
                }))
            .abs()
                < 1e-6
        );

        let dab = dot_product(&a, &b).abs();
        assert!(dab <= calculate_norm(&a) * calculate_norm(&b) + 1e-6);

        let a_plus_b = Vector {
            x: a.x + b.x,
            y: a.y + b.y,
        };
        assert!(calculate_norm(&a_plus_b) <= calculate_norm(&a) + calculate_norm(&b) + 1e-6);
    }

    #[test]
    fn normalize_properties() {
        let v = Vector { x: -0.5, y: 0.25 };
        let u = normalize(&v);
        assert!((calculate_norm(&u) - 1.0).abs() < 1e-6);

        let uu = normalize(&u);
        assert!((uu.x - u.x).abs() < 1e-6 && (uu.y - u.y).abs() < 1e-6);

        if calculate_norm(&v) > 0.0 {
            let cross = v.x * u.y - v.y * u.x;
            assert!(cross.abs() < 1e-6);
        }

        let z = Vector { x: 0.0, y: 0.0 };
        assert_eq!(normalize(&z), z);
    }

    #[test]
    fn lerp_properties() {
        assert!((linear_interpolation(2.0, 8.0, 0.0) - 2.0).abs() < 1e-6);
        assert!((linear_interpolation(2.0, 8.0, 1.0) - 8.0).abs() < 1e-6);
        assert!((linear_interpolation(0.0, 10.0, -0.5) + 5.0).abs() < 1e-6);

        let (a, b, t) = (3.0f32, 7.0f32, 0.3f32);
        assert!((linear_interpolation(a, b, t) - (a + t * (b - a))).abs() < 1e-6);
    }

    #[test]
    fn fade_constraints() {
        assert!((fade(0.0) - 0.0).abs() < 1e-6);
        assert!((fade(1.0) - 1.0).abs() < 1e-6);
        assert!(deriv(fade, 0.0).abs() < 1e-2);
        assert!(deriv(fade, 1.0).abs() < 1e-2);

        let mut prev = fade(0.0);
        for i in 1..=100 {
            let x = i as f32 / 100.0;
            let y = fade(x);
            assert!(y >= prev - 1e-6);
            prev = y;
        }
    }

    #[test]
    fn perlin_is_deterministic() {
        let n = Noise::new(8.0, 1.0, 42);
        let a = n.get(12.345, -6.789);
        let b = n.get(12.345, -6.789);
        assert!((a - b).abs() < 1e-6);
    }

    #[test]
    fn perlin_bounds_reasonable() {
        let n = Noise::new(4.0, 1.0, 7);
        for i in -5..=5 {
            for j in -5..=5 {
                let v = n.get(i as f32 * 0.37, j as f32 * 0.41);
                assert!(v.is_finite());
                assert!(v >= -1.5 && v <= 1.5);
            }
        }
    }

    #[test]
    fn perlin_continuity_c1_local() {
        let n = Noise::new(5.0, 1.0, 1);
        let x = 2.3f32;
        let y = -1.7f32;
        let h = 1e-3f32;
        let c0 = n.get(x, y);
        let cx = n.get(x + h, y);
        let cy = n.get(x, y + h);
        assert!((cx - c0).abs() < 0.1 && (cy - c0).abs() < 0.1);
    }

    #[test]
    fn test_perlin() {
        let n = Noise::new(8.0, 1.0, 42);
        let h = 1e-3;
        let p = n.get(3.2, 4.7);
        assert!((n.get(3.2 + h, 4.7) - p).abs() < 0.1);
        assert!((n.get(3.2, 4.7 + h) - p).abs() < 0.1);
        assert!((n.get(4.0 - h, 4.0) - n.get(4.0 + h, 4.0)).abs() < 0.2);
    }
}

#[cfg(test)]
mod perlin_tests {
    use super::*;

    #[test]
    fn perlin_deterministic() {
        let n = Noise::new(32.0, 1.0, 123);
        let a = n.get(12.34, -5.67);
        let b = n.get(12.34, -5.67);
        assert!((a - b).abs() < 1e-6);
    }

    #[test]
    fn perlin_reasonable_bounds() {
        let n = Noise::new(16.0, 1.0, 7);
        for i in -4..=4 {
            for j in -4..=4 {
                let v = n.get(i as f32 * 0.37, j as f32 * 0.41);
                assert!(v.is_finite());
                assert!(v >= -1.5 && v <= 1.5);
            }
        }
    }

    #[test]
    fn perlin_local_continuity() {
        let n = Noise::new(24.0, 1.0, 42);
        let (x, y) = (3.2f32, -4.7f32);
        let h = 1e-3f32;
        let c0 = n.get(x, y);
        assert!((n.get(x + h, y) - c0).abs() < 0.1);
        assert!((n.get(x, y + h) - c0).abs() < 0.1);
    }
}
