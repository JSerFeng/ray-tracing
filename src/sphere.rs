use super::{
	hittable::{HitRecord, Hittable},
	ray,
	vec3::{dot, Vec3},
};

/**
 * (x - a)^2 + (y - b)^2 + (z - c)^2 = r^2
 */
pub struct Sphere {
	center: Vec3,
	radius: f64,
}

impl Sphere {
	pub fn new(center: Vec3, radius: f64) -> Sphere {
		Sphere { center, radius }
	}
}

#[allow(non_snake_case)]
impl Hittable for Sphere {
	fn hit(&self, ray: &ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
		// A: ray.origin  B: ray.direction  C: center  r: radius
		// 公式: (A + t(未知) * B - C)^2 = r ^ 2
		// t^2 B⋅B+ t * 2B⋅(A−C)+(A−C)^2−r^2=0

		let A = &ray.origin;
		let B = &ray.direction;
		let C = &self.center;
		let r = self.radius;

		let a = dot(B, B);
		let A_C = &(A - C);
		let b = 2.0 * dot(B, A_C);
		let c = dot(A_C, A_C) - r * r;

		let discriminant = b * b - 4.0 * a * c;
		if discriminant < 0.0 {
			return false;
		}
		let root = (-b - discriminant.sqrt()) / 2.0 * a;
		if root < t_min || root > t_max {
			return false;
		}

		let attachment = ray.at(root);

		rec.t = root;
		rec.normal = (&attachment - &self.center).unit_vector();
		rec.p = attachment;
		return true;
	}
}
