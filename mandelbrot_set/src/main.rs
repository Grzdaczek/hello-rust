extern crate image;
extern crate num_complex;

const WIDTH: u32 = 2000;
const HEIGHT: u32 = 2000;

const X: f32 = -0.909;
const Y: f32 = 0.275;
const ZOOM: f32 = WIDTH as f32 * 7.0;
// const X: f32 = -0.5;
// const Y: f32 = 0.0;
// const ZOOM: f32 = WIDTH as f32 * 0.4;

const X_SPAN: f32 = WIDTH as f32 / 2.0 / ZOOM;
const Y_SPAN: f32 = HEIGHT as f32 / 2.0 / ZOOM;

const X_MIN: f32 = X - X_SPAN;
const X_MAX: f32 = X + X_SPAN;
const Y_MIN: f32 = Y - Y_SPAN;
const Y_MAX: f32 = Y + Y_SPAN;

const I_MAX: f32 = 5000.0;

fn main() {

	let mut imgbuf = image::ImageBuffer::new(WIDTH, HEIGHT);
	let mut progress = 0.0;

	for (_x, _y, pixel) in imgbuf.enumerate_pixels_mut() {
		let x = ( X_MAX - X_MIN ) / (WIDTH as f32) * ( _x as f32 - 0.0) + X_MIN;
		let y = ( Y_MAX - Y_MIN ) / (HEIGHT as f32) * (HEIGHT - _y) as f32 * (1.0) + Y_MIN;
		
		// fractal iterations for complex value (x + yi)
		let i = iterate(x, y, 0.0, 0.0);
			// let i = fractal(-0.7269, 0.1889, x, y);

		// color generation
		let hue: f32 = (360.0 * i / I_MAX) as f32;
		let sat: f32 = 1.0;
		let val: f32 = if i < I_MAX { 1.0 } else { 0.0 };
		let (r, g, b) = hsv_to_rgb(hue, sat, val);
		*pixel = image::Rgb([r, g, b]);

		// progress logging
		let p = (_y + 1) as f32 / HEIGHT as f32 * 100.0;
		if progress <= p && p % 1.0 == 0.0 {
			println!("{}%", progress);
			progress += 1.0;
		}
	}

	imgbuf.save("fractal.png").unwrap();
}

fn iterate(c_re: f32, c_im: f32, z_re: f32, z_im: f32) -> f32 {
	let c = num_complex::Complex::new(c_re, c_im);
	let mut z = num_complex::Complex::new(z_re, z_im);
	let mut i: f32 = 0.0;
	while i < I_MAX * 1.0 && z.norm() <= 3.0 {
		z = z * z + c;
		i += 1.0;
	}

	if i == I_MAX	{ return I_MAX as f32;}
	else			{ return i as f32 + 1.0 - z.norm().log2().log10(); }
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
	let c: f32 = v * s;
	let x: f32 = c * ( 1.0 - ( (h/60.0) % 2.0 - 1.0).abs());
	let m: f32 = v - c;
	
	let (r, g, b) = 
		if      h < 60.0	{ (c, x, 0.0) }
		else if h < 120.0	{ (x, c, 0.0) }
		else if h < 180.0	{ (0.0, c, x) }
		else if h < 240.0	{ (0.0, x, c) }
		else if h < 300.0	{ (x, 0.0, c) }
		else 				{ (c, 0.0, x) };


	// (R,G,B) = ((R'+m)×255, (G'+m)×255, (B'+m)×255)
	return ( 
		((r+m)*255.0 ) as u8,
		((g+m)*255.0 ) as u8,
		((b+m)*255.0 ) as u8
	);
}
