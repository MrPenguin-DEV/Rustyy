//! Small, dependency-free mathematical foundation for AETHER.
//!
//! This crate intentionally starts with the SDF operations needed by the first
//! renderer prototype. GPU implementations should mirror these definitions
//! and retain explicit iteration and distance bounds.

use core::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0);
    pub const X: Self = Self::new(1.0, 0.0, 0.0);
    pub const Y: Self = Self::new(0.0, 1.0, 0.0);
    pub const Z: Self = Self::new(0.0, 0.0, 1.0);

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn abs(self) -> Self {
        Self::new(self.x.abs(), self.y.abs(), self.z.abs())
    }

    pub fn max(self, rhs: Self) -> Self {
        Self::new(self.x.max(rhs.x), self.y.max(rhs.y), self.z.max(rhs.z))
    }

    pub fn min(self, rhs: Self) -> Self {
        Self::new(self.x.min(rhs.x), self.y.min(rhs.y), self.z.min(rhs.z))
    }

    pub fn max_component(self) -> f32 {
        self.x.max(self.y).max(self.z)
    }

    pub fn normalized(self) -> Self {
        let len = self.length();
        if len > f32::EPSILON { self / len } else { Self::ZERO }
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self { Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z) }
}
impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self { Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z) }
}
impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self { Self::new(self.x * rhs, self.y * rhs, self.z * rhs) }
}
impl Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self { Self::new(self.x / rhs, self.y / rhs, self.z / rhs) }
}
impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self { Self::new(-self.x, -self.y, -self.z) }
}

/// Exact signed distance to a sphere centered at the origin.
pub fn sd_sphere(point: Vec3, radius: f32) -> f32 {
    point.length() - radius.max(0.0)
}

/// Exact signed distance to an axis-aligned box centered at the origin.
pub fn sd_box(point: Vec3, half_extents: Vec3) -> f32 {
    let q = point.abs() - half_extents.max(Vec3::ZERO);
    q.max(Vec3::ZERO).length() + q.max_component().min(0.0)
}

/// Smoothly blends two signed distance values using polynomial smooth-min.
pub fn smooth_union(a: f32, b: f32, blend_radius: f32) -> f32 {
    let k = blend_radius.max(0.0);
    if k <= f32::EPSILON {
        return a.min(b);
    }
    let h = (0.5 + 0.5 * (b - a) / k).clamp(0.0, 1.0);
    b * (1.0 - h) + a * h - k * h * (1.0 - h)
}

/// Estimates an SDF normal with a tetrahedral finite-difference stencil.
pub fn estimate_normal<F>(point: Vec3, epsilon: f32, distance: F) -> Vec3
where
    F: Fn(Vec3) -> f32,
{
    let e = epsilon.max(1.0e-6);
    let k1 = Vec3::new(1.0, -1.0, -1.0);
    let k2 = Vec3::new(-1.0, -1.0, 1.0);
    let k3 = Vec3::new(-1.0, 1.0, -1.0);
    let k4 = Vec3::new(1.0, 1.0, 1.0);
    (k1 * distance(point + k1 * e)
        + k2 * distance(point + k2 * e)
        + k3 * distance(point + k3 * e)
        + k4 * distance(point + k4 * e))
        .normalized()
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MarchConfig {
    pub max_steps: u32,
    pub max_distance: f32,
    pub hit_epsilon: f32,
    pub min_step: f32,
}

impl Default for MarchConfig {
    fn default() -> Self {
        Self {
            max_steps: 128,
            max_distance: 10_000.0,
            hit_epsilon: 1.0e-3,
            min_step: 1.0e-4,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MarchHit {
    pub distance: f32,
    pub position: Vec3,
    pub steps: u32,
}

/// Bounded sphere tracing. The caller supplies a signed-distance function.
/// Returns None when the ray misses or reaches either configured bound.
pub fn sphere_trace<F>(ray: Ray, config: MarchConfig, distance: F) -> Option<MarchHit>
where
    F: Fn(Vec3) -> f32,
{
    let direction = ray.direction.normalized();
    if direction == Vec3::ZERO || config.max_steps == 0 {
        return None;
    }

    let max_distance = config.max_distance.max(0.0);
    let epsilon = config.hit_epsilon.max(1.0e-7);
    let min_step = config.min_step.max(1.0e-7);
    let mut travel = 0.0;

    for step in 0..config.max_steps {
        let position = ray.origin + direction * travel;
        let d = distance(position);
        if !d.is_finite() || d < -epsilon {
            return None;
        }
        if d <= epsilon {
            return Some(MarchHit {
                distance: travel,
                position,
                steps: step + 1,
            });
        }

        travel += d.max(min_step);
        if travel > max_distance {
            return None;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sphere_distance_is_exact_on_axis() {
        assert!((sd_sphere(Vec3::new(2.0, 0.0, 0.0), 1.0) - 1.0).abs() < 1.0e-6);
        assert!((sd_sphere(Vec3::ZERO, 1.0) + 1.0).abs() < 1.0e-6);
    }

    #[test]
    fn box_distance_is_exact_outside_and_inside() {
        assert!((sd_box(Vec3::new(2.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0)) - 1.0).abs() < 1.0e-6);
        assert!((sd_box(Vec3::ZERO, Vec3::new(1.0, 1.0, 1.0)) + 1.0).abs() < 1.0e-6);
    }

    #[test]
    fn smooth_union_reduces_to_min_without_blend() {
        assert_eq!(smooth_union(2.0, 1.0, 0.0), 1.0);
    }

    #[test]
    fn sphere_trace_hits_sphere() {
        let ray = Ray { origin: Vec3::new(0.0, 0.0, -3.0), direction: Vec3::Z };
        let hit = sphere_trace(ray, MarchConfig::default(), |p| sd_sphere(p, 1.0)).unwrap();
        assert!((hit.distance - 2.0).abs() < 2.0e-3);
    }

    #[test]
    fn sphere_trace_respects_step_bound() {
        let ray = Ray { origin: Vec3::new(0.0, 0.0, -3.0), direction: Vec3::Z };
        let config = MarchConfig { max_steps: 1, ..MarchConfig::default() };
        assert!(sphere_trace(ray, config, |p| sd_sphere(p, 1.0)).is_none());
    }
}
