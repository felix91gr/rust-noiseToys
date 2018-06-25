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

	// use gradient::create_gradient;
	use gradient::create_gradient;

	quickcheck! {
	  fn prop(xs: Vec<f32>) -> bool {

	      create_gradient(xs) != xs
	  }
	}


}
