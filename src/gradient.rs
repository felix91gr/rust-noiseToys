use rand::prelude::*;
use rand::prng::IsaacRng;

use std::hash::Hasher;
use std::collections::hash_map::DefaultHasher;

pub fn create_gradient(corner: &Vec<f32>) -> Vec<f32> {
    //////////////////////////////////////////////
    ///// Step 1: get a hash from the vector /////
    //////////////////////////////////////////////

    // 1.1: Create a hasher
    let mut hasher = DefaultHasher::new();

    // 1.2: Feed the vector's bits to the hasher
    for _x in corner.iter() {
        hasher.write_u32(_x.to_bits());
    }

    // 1.3: Compute the hash
    let hash_of_corner = hasher.finish();

    //////////////////////////////////////////////
    ///// Step 2: seed a prng with that hash /////
    //////////////////////////////////////////////

    // 2.1: Create a seeded PRNG with the hash's bytes
    let mut s_rng: IsaacRng = IsaacRng::new_from_u64(hash_of_corner);

    //////////////////////////////////////////////////////////////////
    ///// Step 3: populate a vector with pseudorandom -1s and 1s /////
    //////////////////////////////////////////////////////////////////

    // 3.1: Initiate the gradient's vector
    let mut seeded_gradient : Vec<f32> = Vec::new();

    // 3.2: Create a factor to make the gradient unitary
    let dimentional_ponderator = 1.0 / f32::sqrt((corner.len() - 1) as f32);

    // 3.3: Fill the gradient with random directions
    for _x in corner.iter() {

        // 3.3.1: Direction must be +1 or -1
        let new_dir = ((s_rng.gen_range(0, 2) as f32) * 2.0) - 1.0;
        
        // 3.3.2: In order to have a unitary gradient, we must use our factor
        let new_dir_ponderated = new_dir * dimentional_ponderator; 

        // 3.3.3: Put the finished direction into the gradient's vector
        seeded_gradient.push(new_dir_ponderated);
    }

    //////////////////////////////////////////////////////////////////////
    ///// Step 4: remove one element from it and return the gradient /////
    //////////////////////////////////////////////////////////////////////

    // 4.1: Compute the zeroed direction for this gradient
    let zeroed_index = s_rng.gen_range(0, corner.len());

    // 4.2: Zero that direction
    seeded_gradient[zeroed_index] = 0.0;

    ///////////////////////
    ///// Return Step /////
    ///////////////////////

    seeded_gradient
}
