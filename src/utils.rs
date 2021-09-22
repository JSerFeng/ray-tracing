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
