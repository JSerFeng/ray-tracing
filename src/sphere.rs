use super::{
	hittable::{HitRecord, Hittable},
	materia, ray,
	vec3::{dot, Vec3},
};

/**
 * (x - a)^2 + (y - b)^2 + (z - c)^2 = r^2
 */
pub struct Sphere<'m> {
	center: Vec3,
	radius: f64,
	pub materia: &'m dyn materia::Materia,
}

impl<'a> Sphere<'a> {
	pub fn new(center: Vec3, radius: f64, materia: &dyn materia::Materia) -> Sphere {
		Sphere {
			center,
			radius,
			materia,
		}
	}
}

#[allow(non_snake_case)]
impl<'a> Hittable for Sphere<'a> {
	fn hit(&self, ray: &ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
		let r = self.radius;
		let oc = &(&ray.origin - &self.center);
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

		let scatted = self.materia.scatter(ray, rec);
		if !scatted {
			rec.albedo = Vec3::zero();
		}

		return true;
	}
}

pub struct SphereList<'m> {
	pub list: Vec<&'m Sphere<'m>>,
}

impl<'a> SphereList<'a> {
	pub fn new(list: Vec<&'a Sphere<'a>>) -> SphereList<'a> {
		SphereList { list }
	}
}

impl<'a> Hittable for SphereList<'a> {
	fn hit(&self, ray: &ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
		let mut has_hit = false;
		let mut tmp_rec = HitRecord::new();
		let mut tmp_t_max = t_max;
		for item in self.list.iter() {
			if item.hit(ray, t_min, tmp_t_max, &mut tmp_rec) {
				tmp_t_max = tmp_rec.t;
				item.materia.scatter(ray, &mut tmp_rec);
				has_hit = true;
			}
		}
		if has_hit {
			*rec = tmp_rec;
		}
		return has_hit;
	}
}
