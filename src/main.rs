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
#[macro_use]
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
	use gradient::create_gradient;

	quickcheck! {
		// Checks that generated gradients are unitary
		fn gradients_are_unitary(xs: Vec<f32>) -> TestResult {

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

		// Checks that generated gradients have the same number
		// of dimensions as the vector they were born from.
		fn gradients_are_of_same_dimensions(xs: Vec<f32>) -> TestResult {
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
	}


}
