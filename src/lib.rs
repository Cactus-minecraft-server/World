pub struct Noise {
    scale: f64,
    amplitude: f64,
}
pub struct Vector {
    x: f32,
    y: f32,
}
impl Noise {
    pub fn new(scale: f64, amplitude: f64) -> Self {
        Self { scale, amplitude }
    }
    pub fn get(&self, x: f64, z: f64) -> f64 {
        let xs = x / self.scale;
        let zs = z / self.scale;
        self.perlin(xs, zs) * self.amplitude
    }
    fn perlin(&self, x: f64, z: f64) -> f64 {
        // implement Perlin noise here (then simplex because it's harder)
        todo!()
    }
}
fn dot_product(v1: Vector, v2: Vector) -> f32 {
    // Calculate the dot product between v1 and v2 using their coordinates.  the result->f32.
    v1.x * v2.x + v1.y * v2.y
}
fn calculate_norm(v1: &Vector) -> f32 {
    // Calculate the norm of a vector using it's coordinates.  the result -> f32.
    (v1.x.powi(2) + v1.y.powi(2)).sqrt()
}
fn normalize(v1: &Vector) -> Vector {
    // This function aim that every vector created randomly has the same norm (1).
    Vector {
        x: v1.x / calculate_norm(v1),
        y: v1.y / calculate_norm(v1),
    }
}
fn fade(x: f32) -> f32 {
    // This is a mathematic equation, it take a value and return the result of the value by that
    // equation. That equation if derivate should be easy when close to 0 and 1 and hard in the
    // middle. Take a f32 value return f32.Both value are between 0 and 1
    6_f32 * x.powi(5) - 15_f32 * x.powi(4) + 10_f32 * x.powi(3)
}
/// Linearly interpolates between two scalar values.
///
/// # Parameters
/// - `a`: start value (when `t = 0.0`)
/// - `b`: end value   (when `t = 1.0`)
/// - `t`: interpolation factor in `[0.0, 1.0]`
///
/// # Returns
/// A value on the line from `a` to `b`:
/// - if `t = 0.0`, yields `a`  
/// - if `t = 1.0`, yields `b`  
/// - otherwise `a + t * (b - a)`
fn linear_interpolation(a: f32, b: f32, t: f32) -> f32 {
    a + t * (b - a)
}
#[cfg(test)]
mod tests {
    use crate::{Vector, calculate_norm, dot_product, linear_interpolation, normalize};
    #[test]
    fn test_dot_product() {
        assert_eq!(
            dot_product(Vector { x: 1.0, y: 0.0 }, Vector { x: 0.0, y: 1.0 }),
            0.0
        );
        assert_eq!(
            dot_product(Vector { x: 1.0, y: 0.5 }, Vector { x: 0.2, y: 1.0 }),
            0.7,
        );
        assert_eq!(
            dot_product(Vector { x: 1.0, y: 0.5 }, Vector { x: -0.2, y: -1.0 }),
            -0.7,
        );
    }
    #[test]
    fn test_calculate_norm() {
        assert_eq!(calculate_norm(&Vector { x: 0.5, y: 0.5 }), 0.5_f32.sqrt());
        assert_eq!(calculate_norm(&Vector { x: 0.7, y: 0.3 }), 0.58_f32.sqrt());
        assert_eq!(
            calculate_norm(&Vector { x: -0.7, y: -0.3 }),
            calculate_norm(&Vector { x: 0.7, y: 0.3 })
        );
    }
    #[test]
    fn test_normalize() {
        let v1 = Vector { x: 0.5, y: 0.5 };
        assert_eq!(calculate_norm(&normalize(&v1)).round(), 1_f32);
    }
    //fn test_fade() {todo!()}
    #[test]
    fn test_linear_interpolation() {
        assert_eq!(linear_interpolation(0.0, 10.0, 0.25), 2.5);
        assert_eq!(linear_interpolation(5.0, 15.0, 0.75), 12.5);
    }
}
