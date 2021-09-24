use super::{
	ray,
	vec3::{dot, Vec3},
};

pub struct HitRecord {
	pub t: f64,
	pub p: Vec3,
	pub normal: Vec3,
	pub albedo: Vec3,
	pub front_face: bool,
	pub out: ray::Ray,
}

impl HitRecord {
	pub fn new() -> HitRecord {
		HitRecord {
			front_face: false,
			t: std::f64::MAX,
			p: Vec3::zero(),
			normal: Vec3::zero(),
			albedo: Vec3::zero(),
			out: ray::Ray::new(Vec3::zero(), Vec3::zero()),
		}
	}

	pub fn set_front_face(&mut self, ray: &ray::Ray, normal: &Vec3) {
		if dot(&ray.direction, normal) > 0.0 {
			self.front_face = false;
			self.normal = -1.0 * normal;
		} else {
			self.normal = Vec3::copy(normal);
			self.front_face = true;
		}
	}
}

pub trait Hittable {
	fn hit(&self, ray: &ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
