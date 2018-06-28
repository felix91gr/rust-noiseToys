use gradient::create_gradient;

mod gradient;

extern crate rand;

fn main() {
    println!("Hello, world!");

    let seeded_gradient = create_gradient(&vec![1., 2., 3.]);

    println!("Random (seeded) numbers:");
    for _x in seeded_gradient {
        println!("> {}", _x);
    }
}

#[cfg(test)]
extern crate quickcheck;

#[cfg(test)]
mod tests {

	// Returns the length of a vector
	// (euclidean metric)
	fn size(xs: &Vec<f32>) -> f32 {
	    let mut total = 0.0;

	    for x in xs {
	        total += x * x;
	    }

	    f32::sqrt(total)
	}

	// Compares two values that are close but
	// not necessarily equal
	fn compare_with_tolerance(lhs: f32, rhs: f32, tolerance: f32) -> bool {
		f32::abs(lhs - rhs) <= tolerance
	}

	use quickcheck::TestResult;
	use quickcheck::QuickCheck;
	use gradient::create_gradient;
	use rand::Rng;
	use rand::prelude::*;


	// Checks that generated gradients are unitary
	// FIXME: compare_with_tolerance is not ideal. Should use
	// a specialized library to compare floats.
	#[test]
	fn gradients_are_unitary() {
		// Test function. We call it using QuickCheck, below.
		fn g_are_u(xs: Vec<f32>) -> TestResult {

			// Unitary gradients for 1D and 0D don't 
			// make sense, at least under how 
			// create_gradient is defined.
			if xs.len() > 1 {

				let gradient = create_gradient(&xs);

				let gradient_size = size(&gradient);

				let res = compare_with_tolerance(1.0, gradient_size, 0.01);
			
				TestResult::from_bool(res)
			}
			else {
			    TestResult::discard()
			}
		}

		QuickCheck::new()
			.tests(10_000)
			.max_tests(1_000_000)
			.quickcheck(g_are_u as fn(Vec<f32>) -> TestResult);
	}

	// Checks that generated gradients have the same number
	// of dimensions as the vector they were born from.
	#[test]
	fn gradients_are_of_same_dimensions() {
		// Test function. We call it using QuickCheck, below.
		fn g_are_of_s_d(xs: Vec<f32>) -> TestResult {
		    // Unitary gradients for 1D and 0D don't 
			// make sense, at least under how 
			// create_gradient is defined.
			if xs.len() > 1 {

				let gradient = create_gradient(&xs);

				let res = gradient.len() == xs.len();
			
				TestResult::from_bool(res)
			}
			else {
			    TestResult::discard()
			}
		}

		QuickCheck::new()
			.tests(1_000)
			.max_tests(10_000)
			.quickcheck(g_are_of_s_d as fn(Vec<f32>) -> TestResult);
	}

	fn get_random_vector_of_length_n(n: usize) -> Vec<f32> {
	    
		let mut rng = thread_rng();

		let mut new_vec : Vec<f32> = Vec::new();

		let limit : f32 = 100_000_000.0;

		for _i in 0..n {
			new_vec.push(rng.gen_range(-limit, limit));
		}

	    new_vec
	}

	fn add_vectors(a : &Vec<f32>, b : &Vec<f32>) -> Vec<f32> {

		assert!(a.len() == b.len());

		let mut sum : Vec<f32> = Vec::new();

		for i in 0..a.len() {
			sum.push(a[i] + b[i]);
		}

		sum
	}

	// Checks that generated gradients have no directional
	// preference.
	#[test]
	fn gradient_mean_is_zero() {

		fn test_for_length_n (n: usize) -> bool {
			if n < 2 {
			    false
			}
			else {
				let size_of_sample = 101;

				println!("Testing for n = {}, with sample size = {}", n, size_of_sample);

				let mut total : Vec<f32> = Vec::new();
				let mut std_dev : Vec<f32> = Vec::new();

				for _i in 0..n {
					total.push(0.0);
					std_dev.push(0.0);
				}

				// Where we store all of them for later
				// estimating the standard deviation 
				let mut all_gradients : Vec<Vec<f32>> = Vec::new();

				for _j in 0..size_of_sample {
					let corner = get_random_vector_of_length_n(n);

					let gradient_of_corner = create_gradient(&corner);

					total = add_vectors(&total, &gradient_of_corner);

					all_gradients.push(gradient_of_corner);
				}

				// We calculate the sample mean
				for i in 0..n {
					total[i] = total[i] / (size_of_sample as f32);
				}

				let deg_of_freedom = size_of_sample - 1;

				// We calculate the sample std_dev
				for i in 0..n {
					for j in 0..size_of_sample {
						let delta_of_sample = total[i] - all_gradients[j][i];
						std_dev[i] += f32::sqrt(delta_of_sample * delta_of_sample);
					}
					std_dev[i] = std_dev[i] / (deg_of_freedom as f32);
					std_dev[i] = f32::sqrt(std_dev[i]);
				}

				// We use t-Student to figure out the confidence interval
				// These values are for 99% confidence
				let t_star : f32 = 
					if deg_of_freedom >= 1_000 {
					    2.581
					}
					else if deg_of_freedom >= 100 {
					    2.626
					}
					else if deg_of_freedom >= 80 {
						2.639
					}
					else if deg_of_freedom >= 60 {
					    2.660
					}
					else {
					    2.704
					};

				let mut lower_bounds : Vec<f32> = Vec::new();
				let mut upper_bounds : Vec<f32> = Vec::new();

				let root_n = f32::sqrt(n as f32);

				for i in 0..n {
					let delta_of_interval = t_star * std_dev[i] / root_n;
				    lower_bounds.push(total[i] - delta_of_interval);
				    upper_bounds.push(total[i] + delta_of_interval);
				}

				let mut they_were_within_margin = true;

				for i in 0..n {
					let this_zero_is_within_margin = lower_bounds[i] <= 0.0 && 0.0 <= upper_bounds[i];

					println!("Bounds for 99% confidence: {}..{}", lower_bounds[i], upper_bounds[i]);

					they_were_within_margin = they_were_within_margin && this_zero_is_within_margin;
				}

				they_were_within_margin
			}
		}

		let max_dimensions : usize = 40;

		println!("Testing for dimensions from n = 2 to n = {}", max_dimensions);

		for n in 2..max_dimensions {

			let result_for_n = test_for_length_n(n);

			if !result_for_n {
				panic!("Failed with n = {}", n);
			}
		}
	}
}
