pub mod camera;
pub mod hittable;
pub mod materia;
pub mod ray;
pub mod scene;
pub mod sphere;
pub mod utils;
pub mod vec3;

use std::io::Write;
const RATIO: f64 = 16.0 / 9.0;
const H: f64 = 256.0;
const W: f64 = RATIO * H;
const MAX_REFLECTED_TIMES: u32 = 8;
const MSAA_SAMPLES: u32 = 10;

fn main() {
	let camera = camera::Camera::new(
		vec3::Vec3::new(0.0, 2.0, 1.0),
		vec3::Vec3::new(0.0, 0.0, -2.0),
		vec3::Vec3::new(0.0, 1.0, 0.0),
		60.0,
		RATIO,
		2.0,
		3.0,
	);

	let mut data: Vec<u32> = Vec::with_capacity(W as usize * H as usize * 3);

	let mut l_list: Vec<materia::Lambertian> = Vec::new();
	for _ in 0..25 {
		l_list.push(materia::Lambertian::new(vec3::Vec3::rand(0.0, 1.0)));
	}
	let mut m_list: Vec<materia::Metal> = Vec::new();
	for _ in 0..25 {
		m_list.push(materia::Metal::new(
			vec3::Vec3::rand(0.0, 1.0),
			utils::random(0.0, 1.0),
		));
	}
	let mut d_list: Vec<materia::Dieletric> = Vec::new();
	for _ in 0..25 {
		d_list.push(materia::Dieletric::new(utils::random(1.0, 2.0)));
	}
	let ground_mat: materia::Lambertian = materia::Lambertian::new(vec3::Vec3::new(0.7, 0.2, 0.5));
	let list = scene::random_scene(&l_list, &m_list, &d_list, &ground_mat);

	for ih in 0..H as u32 {
		println!("{0:<.2}", (ih as f64 / H * 100.0));
		for iw in 0..W as u32 {
			let mut result_color = vec3::Vec3::zero();
			//MSAA
			for _ in 0..MSAA_SAMPLES {
				let s = (iw as f64 + utils::random_num::<f64>()) / (W - 1.0);
				let t = ((H - ih as f64) + utils::random_num::<f64>()) / (H - 1.0);
				let ray = camera.get_ray(s, t);
				result_color = &result_color + &ray_color(&ray, &list, MAX_REFLECTED_TIMES);
			}

			result_color = &result_color / MSAA_SAMPLES as f64;

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
	if depth <= 0 {
		return vec3::Vec3::zero();
	}
	let rec = &mut hittable::HitRecord::new();
	if hittable_obj.hit(&ray, 0.01, std::f64::MAX, rec) {
		//??????????????????????????????
		return &rec.albedo * &ray_color(&rec.out, hittable_obj, depth - 1);
	}
	let t = 0.5 * (ray.direction.unit_vector().y() + 1.0);
	let color = utils::interp(
		t,
		&vec3::Vec3::new(0.5, 0.7, 1.0),
		&vec3::Vec3::new(1.0, 1.0, 1.0),
	);
	return color;
}
