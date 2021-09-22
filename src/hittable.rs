use super::{
	ray,
	vec3::{dot, Vec3},
};

pub struct HitRecord {
	pub t: f64,
	pub p: Vec3,
	pub normal: Vec3,
	pub front_face: bool,
}

impl HitRecord {
	pub fn new() -> HitRecord {
		HitRecord {
			front_face: false,
			t: std::f64::MAX,
			p: Vec3::zero(),
			normal: Vec3::zero(),
		}
	}

	pub fn set_front_face(&mut self, ray: &ray::Ray, normal: &Vec3) {
		if dot(&ray.direction, normal) > 0.0 {
			self.front_face = false;
		} else {
			self.front_face = true;
		}
	}
}

pub trait Hittable {
	fn hit(&self, ray: &ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
