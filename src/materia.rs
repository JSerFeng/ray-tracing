use super::{
	hittable::HitRecord,
	new_vec3,
	ray::Ray,
	utils,
	vec3::{dot, reflect, refract, Vec3},
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
	fuzz: f64,
}

impl Metal {
	pub fn new(albedo: Vec3, fuzz: f64) -> Metal {
		Metal { albedo, fuzz }
	}
}

impl Materia for Metal {
	fn scatter(&self, ray: &Ray, rec: &mut HitRecord) -> bool {
		let reflected = reflect(&ray.direction.unit_vector(), &rec.normal);
		let ray_out = Ray::new(
			Vec3::copy(&rec.p),
			&reflected + &(self.fuzz * &utils::random_in_unit_sphere()),
		);
		rec.albedo = Vec3::copy(&self.albedo);
		rec.out = ray_out;
		return dot(&reflected, &rec.normal) > 0.0;
	}
}

pub struct Dieletric {
	ir: f64,
}

impl Dieletric {
	pub fn new(ir: f64) -> Dieletric {
		Dieletric { ir }
	}
}

impl Materia for Dieletric {
	fn scatter(&self, ray: &Ray, rec: &mut HitRecord) -> bool {
		let refraction_ratio = if rec.front_face {
			1.0 / self.ir
		} else {
			self.ir
		};
		let unit_direction = ray.direction.unit_vector();

		let cos_theta = dot(&(-1.0 * &ray.direction.unit_vector()), &rec.normal).min(1.0);
		let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

		let cannot_refract = refraction_ratio * sin_theta > 1.0;
		let direction = if cannot_refract {
			reflect(&unit_direction, &rec.normal)
		} else {
			refract(&unit_direction, &rec.normal, refraction_ratio)
		};

		rec.out = Ray::new(Vec3::copy(&rec.p), direction);
		rec.albedo = new_vec3![1.0];
		return true;
	}
}
