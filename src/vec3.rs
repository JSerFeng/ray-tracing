use std::ops;

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
		self.len_squre().sqrt()
	}

	pub fn len_squre(&self) -> f64 {
		self.value[0] * self.value[0] + self.value[1] * self.value[1] + self.value[2] * self.value[2]
	}

	pub fn unit_vector(&self) -> Vec3 {
		self / self.len()
	}
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
