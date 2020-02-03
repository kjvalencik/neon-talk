// https://github.com/rayon-rs/rayon/blob/master/rayon-demo/src/sieve/mod.rs
use rayon::prelude::*;

const CHUNK_SIZE: usize = 100_000;

fn clear_stride(slice: &mut [bool], from: usize, stride: usize) {
	let slice = &mut slice[from..];

	for x in slice.iter_mut().step_by(stride) {
		*x = false;
	}
}

fn sieve_serial(max: usize) -> Vec<bool> {
	let mut sieve = vec![true; max / 2];

	sieve[0] = false; // 1 is not prime

	for i in 1.. {
		if sieve[i] {
			let p = 2 * i + 1;
			let pp = p * p;
			if pp >= max {
				break;
			}
			clear_stride(&mut sieve, pp / 2, p);
		}
	}
	sieve
}

/// Update a chunk with low primes
fn update_chunk(low: &[bool], chunk: &mut [bool], base: usize) {
	let max = base + chunk.len() * 2;
	for (i, &is_prime) in low.iter().enumerate() {
		if is_prime {
			let p = 2 * i + 1;
			let pp = p * p;
			if pp >= max {
				break;
			}

			let pm = if pp < base {
				// p² is too small - find the first odd multiple that's in range
				(((base + p - 1) / p) | 1) * p
			} else {
				pp
			};

			if pm < max {
				clear_stride(chunk, (pm - base) / 2, p);
			}
		}
	}
}

pub fn sieve(max: usize) -> impl Iterator<Item = usize> {
	// first compute the small primes, up to sqrt(max).
	let small_max = (max as f64).sqrt().ceil() as usize;
	let mut sieve = sieve_serial(small_max);

	sieve.resize(max / 2, true);

	{
		let (low, high) = sieve.split_at_mut(small_max / 2);
		high.par_chunks_mut(CHUNK_SIZE)
			.enumerate() // to figure out where this chunk came from
			.with_max_len(1) // ensure every single chunk is a separate rayon job
			.for_each(|(chunk_index, chunk)| {
				let i = small_max / 2 + chunk_index * CHUNK_SIZE;
				let base = i * 2 + 1;
				update_chunk(low, chunk, base);
			});
	}

	sieve
		.into_iter()
		.enumerate()
		.filter_map(|(i, n)| if n { Some(i) } else { None })
}
