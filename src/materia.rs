use super::{
	hittable::HitRecord,
	ray::Ray,
	utils,
	vec3::{dot, reflect, Vec3},
};

pub trait Materia {
	fn scatter(&self, r_in: &Ray, rec: &mut HitRecord) -> bool;
}

pub struct Lambertian {
	albedo: Vec3,
}

impl Lambertian {
	pub fn new(albedo: Vec3) -> Lambertian {
		Lambertian { albedo }
	}
}

impl Materia for Lambertian {
	fn scatter(&self, _: &Ray, rec: &mut HitRecord) -> bool {
		let mut scatted = &rec.normal + &utils::random_unit_vector();
		if utils::near_zero(&scatted) {
			scatted = Vec3::copy(&rec.normal);
		}

		let r_out = Ray::new(Vec3::copy(&rec.p), scatted);
		rec.out = r_out;
		rec.albedo = Vec3::copy(&self.albedo);

		return true;
	}
}

pub struct Metal {
	albedo: Vec3,
}

impl Metal {
	pub fn new(albedo: Vec3) -> Metal {
		Metal { albedo }
	}
}

impl Materia for Metal {
	fn scatter(&self, ray: &Ray, rec: &mut HitRecord) -> bool {
		let reflected = reflect(&ray.direction, &rec.normal);
		let ray_out = Ray::new(Vec3::copy(&rec.p), Vec3::copy(&reflected));
		rec.albedo = Vec3::copy(&self.albedo);
		rec.out = ray_out;
		return dot(&reflected, &rec.normal) > 0.0
	}
}
