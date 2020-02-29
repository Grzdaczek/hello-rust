extern crate image;
extern crate num_complex;

const WIDTH: u32 = 1920 * 1;
const HEIGHT: u32 = 1080 * 1;

const X: f32 = -0.5;
const Y: f32 = 0.0;
const ZOOM: f32 = WIDTH as f32 * 0.2;

const X_SPAN: f32 = WIDTH as f32 / 2.0 / ZOOM;
const Y_SPAN: f32 = HEIGHT as f32 / 2.0 / ZOOM;

const X_MIN: f32 = X - X_SPAN;
const X_MAX: f32 = X + X_SPAN;
const Y_MIN: f32 = Y - Y_SPAN;
const Y_MAX: f32 = Y + Y_SPAN;

const I_MAX: f32 = 3000.0;

fn main() {

	let mut imgbuf = image::ImageBuffer::new(WIDTH, HEIGHT);
	let pixels = fractal_pixels();

	for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
		let i = x * HEIGHT + y;
		let (r, g, b) = pixels[i as usize];
		// println!("{:?}", (r, g, b));
		*pixel = image::Rgb([r, g, b]);
	}

	imgbuf.save("fractal.png").unwrap();
	println!("100%");
}

fn fractal_pixels() -> Vec<(u8, u8, u8)> {
	let mut set: Vec<f32> = vec![];
	let mut progress = 0.0;
	let mut histogram = [0u32; I_MAX as usize];
	let mut total = 0u32;

	for _x in 0..WIDTH {
		for _y in 0..HEIGHT {
			
			let x = ( X_MAX - X_MIN ) / (WIDTH as f32) * ( _x as f32 - 0.0) + X_MIN;
			let y = ( Y_MAX - Y_MIN ) / (HEIGHT as f32) * (HEIGHT - _y) as f32 * (1.0) + Y_MIN;
			
			// fractal iterations for complex value (x + yi)
			let i = fractal_point(x, y, 0.0, 0.0);

			// add to histogram
			if i < I_MAX {
				histogram[i.floor() as usize] += 1;
				total += 1;
			}
			
			
			set.push(i);
		}
		
		// progress logging
		let p = _x as f32 / WIDTH as f32 * 100.0;
		if progress + 1.0 <= p {
			println!("{}%", progress);
			progress += 1.0;
		}
	}

	let mut pixels: Vec<(u8, u8, u8)> = vec![];
	let mut hues: Vec<f32> = vec![];
	let mut h = 0f32;

	for x in 0..I_MAX as usize {
		h += histogram[x] as f32 / total as f32;
		hues.push(h);
	}
	hues.push(h);

	for x in 0..set.len() as usize {
		let i = set[x];
		let c1 = hues[i.floor() as usize];
		let c2 = hues[i.ceil() as usize];
		let t = i % 1.0;
		let hue: f32 = 360.0 - 360.0 * ( c1 * (1.0 - t) + c2 * t);
		let sat: f32 = 1.0;
		let val: f32 = if i < I_MAX { 1.0 } else { 0.0 };
		pixels.push(hsv_to_rgb(hue, sat, val));
	}
	return pixels;
}

fn fractal_point(c_re: f32, c_im: f32, z_re: f32, z_im: f32) -> f32 {
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
