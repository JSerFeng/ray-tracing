pub mod camera;
pub mod hittable;
pub mod ray;
pub mod sphere;
pub mod utils;
pub mod vec3;

use std::io::Write;
const RATIO: f64 = 16.0 / 9.0;
const H: f64 = 256.0;
const W: f64 = RATIO * H;
const MAX_REFLECTED_TIMES: u32 = 100;
const MSAA_SAMPLES: u32 = 10;

fn main() {
	let camera = camera::new(RATIO, 2.0);

	let sphere_small = sphere::Sphere::new(vec3::Vec3::new(0.0, 0.0, -1.0), 0.5);
	let sphere_big = sphere::Sphere::new(vec3::Vec3::new(0.0, -100.5, -1.0), 100.0);
	let sphere_list = sphere::SphereList::new(vec![&sphere_small, &sphere_big]);

	let mut data: Vec<u32> = Vec::new();

	for ih in 0..H as u32 {
		for iw in 0..W as u32 {
			let mut result_color = vec3::Vec3::zero();
			//MSAA
			for _ in 1..MSAA_SAMPLES {
				let (x, y) = camera.transform(
					(iw as f64 + utils::random_num::<f64>()) / W * camera.w,
					((H - ih as f64) as f64 + utils::random_num::<f64>()) / H * camera.h,
				);
				let ray = ray::new(vec3::Vec3::new(0.0, 0.0, 0.0), vec3::Vec3::new(x, y, -1.0));
				result_color = &result_color + &ray_color(&ray, &sphere_list, MAX_REFLECTED_TIMES);
			}

			result_color = &result_color / (MSAA_SAMPLES + 1) as f64;

			data.push((result_color.x().sqrt() * 255.0) as u32);
			data.push((result_color.y().sqrt() * 255.0) as u32);
			data.push((result_color.z().sqrt() * 255.0) as u32);
		}
	}
	write2img(&data, W as u32, H as u32);
}

fn write2img(data: &Vec<u32>, w: u32, h: u32) {
	let data_str = utils::to_rgb_str(data, w, h);
	let mut img = std::fs::File::create("img.ppm").unwrap();
	img.write_all(data_str.as_bytes()).unwrap();
}

fn ray_color(ray: &ray::Ray, hittable_obj: &dyn hittable::Hittable, depth: u32) -> vec3::Vec3 {
	if depth == 0 {
		return vec3::Vec3::zero();
	}
	let rec = &mut hittable::HitRecord::new();
	if hittable_obj.hit(&ray, 0.0, std::f64::MAX, rec) {
		//随机模拟光线的漫反射
		let target = &(&rec.p + &rec.normal) + &vec3::Vec3::rand_unit_vec();
		let next_ray = ray::Ray::new(
			vec3::Vec3::copy(&rec.p),
			&target - &vec3::Vec3::copy(&rec.p),
		);
		return 0.5 * &ray_color(&next_ray, hittable_obj, depth - 1);
	}
	let t = 0.5 * (ray.direction.unit_vector().y() + 1.0);
	let color = utils::interp(
		t,
		&vec3::Vec3::new(0.5, 0.7, 1.0),
		&vec3::Vec3::new(1.0, 1.0, 1.0),
	);
	return color;
}
