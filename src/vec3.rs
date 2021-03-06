use super::utils;
use std::ops;

#[macro_export]
macro_rules! new_vec3 {
	() => {
		Vec3::new(0.0, 0.0, 0.0)
	};
	($ele: expr) => {
		Vec3::new($ele, $ele, $ele)
	};
	($ele1: expr, $ele2: expr, $ele3: expr) => {
		Vec3::new($ele1, $ele2, $ele3)
	};
}

pub struct Vec3 {
	value: [f64; 3],
}

impl Vec3 {
	pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
		Vec3 { value: [x, y, z] }
	}

	pub fn from(val: f64) -> Vec3 {
		Vec3::new(val, val, val)
	}

	pub fn rand(l: f64, r: f64) -> Vec3 {
		Vec3::new(
			utils::random(l, r),
			utils::random(l, r),
			utils::random(l, r),
		)
	}

	pub fn copy(v: &Vec3) -> Vec3 {
		Vec3::new(v.x(), v.y(), v.z())
	}

	pub fn zero() -> Vec3 {
		Vec3::new(0.0, 0.0, 0.0)
	}

	pub fn x(&self) -> f64 {
		self.value[0]
	}

	pub fn y(&self) -> f64 {
		self.value[1]
	}

	pub fn z(&self) -> f64 {
		self.value[2]
	}

	pub fn len(&self) -> f64 {
		self.len_square().sqrt()
	}

	pub fn len_square(&self) -> f64 {
		self.value[0] * self.value[0] + self.value[1] * self.value[1] + self.value[2] * self.value[2]
	}

	pub fn unit_vector(&self) -> Vec3 {
		self / self.len()
	}
}

/**
	 r
	 \  n	/|
		\ | / | a
		 \|/	 |
		 ---------------
			 \	 |
			r \ | a
				 \|
*/
pub fn reflect(r: &Vec3, n: &Vec3) -> Vec3 {
	return r - &(2.0 * &(dot(r, n) * n));
}

pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
	let cos_theta = dot(&(-1.0 * uv), n).min(1.0);
	let r_out_perp = etai_over_etat * &(uv + &(cos_theta * n));
	let r_out_parallel = -(1.0 - r_out_perp.len_square()).abs().sqrt() * n;
	&r_out_perp + &r_out_parallel
}

pub fn dot(v1: &Vec3, v: &Vec3) -> f64 {
	return v1.value[0] * v.value[0] + v1.value[1] * v.value[1] + v1.value[2] * v.value[2];
}

pub fn cross(v1: &Vec3, v: &Vec3) -> Vec3 {
	Vec3 {
		value: [
			v1.value[1] * v.value[2] - v1.value[2] * v.value[1],
			v1.value[2] * v.value[0] - v1.value[0] * v.value[2],
			v1.value[0] * v.value[1] - v1.value[1] * v.value[0],
		],
	}
}

impl ops::Add<&Vec3> for &Vec3 {
	type Output = Vec3;

	fn add(self, rhs: &Vec3) -> Vec3 {
		Vec3::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
	}
}

impl ops::Add<f64> for &Vec3 {
	type Output = Vec3;

	fn add(self, rhs: f64) -> Vec3 {
		Vec3::new(self.x() + rhs, self.y() + rhs, self.z() + rhs)
	}
}

impl ops::Sub<&Vec3> for &Vec3 {
	type Output = Vec3;

	fn sub(self, rhs: &Vec3) -> Vec3 {
		Vec3::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
	}
}

impl ops::Sub<f64> for &Vec3 {
	type Output = Vec3;

	fn sub(self, rhs: f64) -> Vec3 {
		Vec3::new(self.x() - rhs, self.y() - rhs, self.z() - rhs)
	}
}

impl ops::Mul<&Vec3> for &Vec3 {
	type Output = Vec3;

	fn mul(self, rhs: &Vec3) -> Vec3 {
		Vec3::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
	}
}

impl ops::Mul<f64> for &Vec3 {
	type Output = Vec3;

	fn mul(self, rhs: f64) -> Vec3 {
		Vec3::new(self.x() * rhs, self.y() * rhs, self.z() * rhs)
	}
}

impl ops::Mul<&Vec3> for f64 {
	type Output = Vec3;

	fn mul(self, rhs: &Vec3) -> Vec3 {
		Vec3::new(rhs.x() * self, rhs.y() * self, rhs.z() * self)
	}
}

impl ops::Div<&Vec3> for &Vec3 {
	type Output = Vec3;

	fn div(self, rhs: &Vec3) -> Vec3 {
		Vec3::new(self.x() / rhs.x(), self.y() / rhs.y(), self.z() / rhs.z())
	}
}

impl ops::Div<f64> for &Vec3 {
	type Output = Vec3;

	fn div(self, rhs: f64) -> Vec3 {
		Vec3::new(self.x() / rhs, self.y() / rhs, self.z() / rhs)
	}
}
