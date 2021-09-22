pub fn new(ratio: f64, h: f64) -> Camera {
	let w = ratio * h;
	Camera { ratio, h, w }
}

pub struct Camera {
	pub ratio: f64,
	pub h: f64,
	pub w: f64,
}

impl Camera {
	pub fn transform(&self, u: f64, v: f64) -> (f64, f64) {
		let x = u - self.w / 2.0;
		let y = v - self.h / 2.0;
		(x, y)
	}
}
