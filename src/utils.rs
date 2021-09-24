use super::vec3;
extern crate rand;

pub fn to_rgb_str(data: &Vec<u32>, w: u32, h: u32) -> String {
	let len = data.len();
	let mut result = String::from(format!("P3\n{} {}\n255\n", w, h));

	let mut i = 0;
	while i < len {
		let r = data[i];
		let g = data[i + 1];
		let b = data[i + 2];

		result.push_str(&format!("{} {} {}\n", r, g, b)[..]);
		i += 3;
	}

	return result;
}

pub fn clamp<T: PartialOrd + Copy>(value: T, min: T, max: T) -> T {
	if value < min {
		return min;
	} else if value > max {
		return max;
	} else {
		return value;
	}
}

pub fn interp(amount: f64, l: &vec3::Vec3, r: &vec3::Vec3) -> vec3::Vec3 {
	let l_calc = l * amount;
	let r_calc = r * (1.0 - amount);
	return &l_calc + &r_calc;
}

pub fn random(l: f64, r: f64) -> f64 {
	let v = rand::random::<f64>();
	v * (r - l) + l
}

pub fn random_num<T: rand::Rand>() -> T {
	rand::random::<T>()
}

pub fn random_in_hemisphere(normal: &vec3::Vec3) -> vec3::Vec3 {
	let v = random_in_unit_sphere();
	if vec3::dot(normal, &v) > 0.0 {
		return v;
	} else {
		return &v * -1.0;
	}
}

pub fn random_unit_vector() -> vec3::Vec3 {
	random_in_unit_sphere().unit_vector()
}

pub fn random_in_unit_sphere() -> vec3::Vec3 {
	loop {
		let v = vec3::Vec3::rand(-1.0, 1.0);
		if v.len_square() < 1.0 {
			return v;
		}
		continue;
	}
}

pub fn near_zero(v: &vec3::Vec3) -> bool {
	return v.x() < 1e-8 && v.y() < 1e-8 && v.z() < 1e-8;
}
