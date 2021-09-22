use super::{ray, vec3::Vec3};

pub struct HitRecord {
	pub color: Vec3,
	pub t: f64,
	pub p: Vec3,
	pub normal: Vec3,
}

impl HitRecord {
	pub fn new() -> HitRecord {
		HitRecord {
			color: Vec3::zero(),
			t: std::f64::MAX,
			p: Vec3::zero(),
			normal: Vec3::zero(),
		}
	}
}

pub trait Hittable {
	fn hit(&self, ray: &ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
