use crate::{
	ray::Ray,
	vec3::{cross, Vec3},
};

pub struct Camera {
	origin: Vec3,
	lower_left_corner: Vec3,
	horizontal: Vec3,
	vertical: Vec3,
	// w: Vec3,
	u: Vec3,
	v: Vec3,
	lens_radius: f64,
}

impl Camera {
	pub fn new(
		look_from: Vec3,
		look_at: Vec3,
		up: Vec3,
		vertical: f64,
		aspect_ratio: f64,
		aperture: f64,
		focus_distance: f64,
	) -> Self {
		let theta = vertical.to_radians();
		let h = (theta / 2.0).tan();
		let viewport_height = 2.0 * h;
		let viewport_width = aspect_ratio * viewport_height;

		let w = (&look_from - &look_at).unit_vector();
		let u = cross(&up, &w).unit_vector();
		let v = cross(&w, &u);

		let origin = look_from;
		let horizontal = focus_distance * viewport_width * &u;
		let vertical = focus_distance * viewport_height * &v;
		let lower_left_corner =
			&(&(&origin - &(&horizontal / 2.0)) - &(&vertical / 2.0)) - &(focus_distance * &w);
		let lens_radius = aperture / 2.0;

		Self {
			origin,
			lower_left_corner,
			horizontal,
			vertical,
			// w,
			u,
			v,
			lens_radius,
		}
	}

	pub fn get_ray(&self, s: f64, t: f64) -> Ray {
		Ray::new(
			Vec3::copy(&self.origin),
			&(&(&self.lower_left_corner + &(s * &self.horizontal)) + &(t * &self.vertical))
				- &self.origin,
		)
	}
}
