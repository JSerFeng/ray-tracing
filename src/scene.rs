use crate::{materia, sphere, utils, vec3::Vec3};


pub fn random_scene<'a>(
	lambertian_list: &'a Vec<materia::Lambertian>,
	metal_list: &'a Vec<materia::Metal>,
	dieletric_list: &'a Vec<materia::Dieletric>,
	ground_mat: &'a materia::Lambertian,
) -> sphere::SphereList<'a> {
	let len1 = lambertian_list.len();
	let len2 = metal_list.len();
	let len3 = dieletric_list.len();
	let total = len1 + len2 + len3;
	let mut v: Vec<sphere::Sphere> = Vec::with_capacity(total);
	for i in 0..total {
		let random_mat: &'a dyn materia::Materia = if i < len1 {
			&lambertian_list[i]
		} else if i < len1 + len2 {
			&metal_list[i - len1]
		} else {
			&dieletric_list[i - len1 - len2]
		};

		let random_radius = utils::random(0.2, 0.5);
		v.push(sphere::Sphere::new(
			Vec3::new(
				utils::random(-10.0, 10.0),
				random_radius,
				utils::random(-10.0, 0.0),
			),
			random_radius,
			random_mat,
		));
	}
	v.push(sphere::Sphere::new(
		Vec3::new(0.0, -1000.0, -1.0),
		1000.0,
		ground_mat,
	));
	let s_list = sphere::SphereList::new(v);
	s_list
}
