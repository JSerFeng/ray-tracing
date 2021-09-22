use super::vec3;

pub struct Ray {
	pub origin: vec3::Vec3,
	pub direction: vec3::Vec3,
}

pub fn new(origin: vec3::Vec3, direction: vec3::Vec3) -> Ray {
	Ray { origin, direction }
}

impl Ray {
	pub fn at(&self, t: f64) -> vec3::Vec3 {
		return &self.origin + &(&self.direction * t);
	}

	pub fn new(origin: vec3::Vec3, direction: vec3::Vec3) -> Ray {
		Ray { origin, direction }
	}
}
