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

		let r = self.radius;
		let oc = &(&ray.origin - &self.center);
		// let a = dot(B, B);
		// let b = 2.0 * dot(B, A_C);
		// let c = dot(A_C, A_C) - r * r;
		// more efficient is as following
		let a = ray.direction.len_square();
		let half_b = dot(oc, &ray.direction);
		let c = oc.len_square() - r * r;

		let discriminant = half_b * half_b - a * c;
		if discriminant < 0.0 {
			return false;
		}
		let root = (-half_b - discriminant.sqrt()) / a;
		if root < t_min || root > t_max {
			return false;
		}

		let attachment = ray.at(root);

		rec.t = root;
		rec.normal = (&attachment - &self.center).unit_vector();
		rec.set_front_face(ray, &(&attachment - &self.center).unit_vector());
		rec.p = attachment;

		return rec.front_face;
	}
}

pub struct SphereList<'a> {
	pub list: Vec<&'a Sphere>,
}

impl<'a> SphereList<'a> {
	pub fn new(list: Vec<&'a Sphere>) -> SphereList {
		SphereList { list }
	}
}

impl Hittable for SphereList<'_> {
	fn hit(&self, ray: &ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
		let mut has_hit = false;
		let mut tmp_rec = HitRecord::new();
		let mut tmp_t_max = t_max;
		for item in self.list.iter() {
			if item.hit(ray, t_min, tmp_t_max, &mut tmp_rec) {
				tmp_t_max = tmp_rec.t;
				has_hit = true;
			}
		}
		if has_hit {
			*rec = tmp_rec;
		}
		return has_hit;
	}
}
