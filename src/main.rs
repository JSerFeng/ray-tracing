pub mod camera;
pub mod hittable;
pub mod ray;
pub mod sphere;
pub mod utils;
pub mod vec3;

use std::io::Write;

fn main() {
	const RATIO: f64 = 16.0 / 9.0;
	const H: f64 = 256.0;
	const W: f64 = RATIO * H;

	let camera = camera::new(RATIO, 2.0);
	let sphere = sphere::Sphere::new(vec3::Vec3::new(0.0, 0.0, -1.0), 0.5);
	let mut rec = hittable::HitRecord::new();
	let mut data: Vec<u32> = Vec::new();

	for ih in 0..H as u32 {
		for iw in 0..W as u32 {
			let (x, y) = camera.transform(
				iw as f64 / W * camera.w,
				((H - ih as f64) as f64) / H * camera.h,
			);

			let ray = ray::new(vec3::Vec3::new(0.0, 0.0, 0.0), vec3::Vec3::new(x, y, -1.0));

			let color = ray_color(&ray, &mut rec, &sphere);
			data.push((color.x() * 255.0) as u32);
			data.push((color.y() * 255.0) as u32);
			data.push((color.z() * 255.0) as u32);
		}
	}
	write2img(&data, W as u32, H as u32);
}

fn write2img(data: &Vec<u32>, w: u32, h: u32) {
	let data_str = utils::to_rgb_str(data, w, h);
	let mut img = std::fs::File::create("img.ppm").unwrap();
	img.write_all(data_str.as_bytes()).unwrap();
}

fn ray_color(
	ray: &ray::Ray,
	rec: &mut hittable::HitRecord,
	hittable_obj: &dyn hittable::Hittable,
) -> vec3::Vec3 {
	if hittable_obj.hit(&ray, std::f64::MIN, std::f64::MAX, rec) {
		return vec3::Vec3::copy(&rec.color);
	}
	let t = 0.5 * (ray.direction.unit_vector().y() + 1.0);
	let color = utils::interp(
		t,
		&vec3::Vec3::new(0.5, 0.7, 1.0),
		&vec3::Vec3::new(1.0, 1.0, 1.0),
	);
	return color;
}
