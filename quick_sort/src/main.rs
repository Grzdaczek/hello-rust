extern crate rand;
use std::time::{Instant};
use rand::thread_rng;
use rand::seq::SliceRandom;

fn main() {
	let vec = get_random_vec();
	println!("Sort vector of {:?} numbers", vec.len());
	
	benchmark(bouble_sort, "Bouble sort", &vec);
	benchmark(native_sort, "Native sort", &vec);
}

fn get_random_vec() -> Vec<u32> {
    let mut vec: Vec<u32> = (0..5000).collect();
    vec.shuffle(&mut thread_rng());
	return vec
}

fn benchmark(f: fn(&mut Vec<u32>), msg: &str, _vec: &Vec<u32>) {
	let mut vec = _vec.clone();
	let now = Instant::now();
	f(&mut vec);
	let elapsed = now.elapsed();
	println!("{}:	{:?}", msg, elapsed);
}

fn bouble_sort(vec: &mut Vec<u32>) {
	loop {
		let mut acted = false;
		for x in 0..(vec.len() - 1) {
			if vec[x] > vec[x+1] {
				vec.swap(x, x+1);
				acted = true;
			}
		}
		if !acted { break; }
	}
}

fn native_sort(vec: &mut Vec<u32>) {
	vec.sort();
}