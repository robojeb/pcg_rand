extern crate pcg_rand;
extern crate rand;

use rand::{Rng, SeedableRng, thread_rng};
use pcg_rand::Pcg32Unique;


const NUM_TESTS : usize = 1000;

#[test]
#[should_panic]
fn pcg32_unique_unseeded() {
    let mut ra : Pcg32Unique = Pcg32Unique::new_unseeded();
    let mut rb : Pcg32Unique = Pcg32Unique::new_unseeded();
    //Because these are unique these should not match
    assert_eq!(ra.gen_ascii_chars().take(100).collect::<Vec<_>>(),
               rb.gen_ascii_chars().take(100).collect::<Vec<_>>());
}

#[test]
#[should_panic]
fn pcg32_unique_seed_match() {
    for _ in 0..NUM_TESTS {
        let s = thread_rng().gen();
        let mut ra : Pcg32Unique = SeedableRng::from_seed(s);
        let mut rb : Pcg32Unique = SeedableRng::from_seed(s);
        //Because these are unique these should not match
        assert_eq!(ra.gen_ascii_chars().take(100).collect::<Vec<_>>(),
                   rb.gen_ascii_chars().take(100).collect::<Vec<_>>());
    }
}

#[test]
fn pcg32_unique_seed_diff() {
    for _ in 0..NUM_TESTS {
        //Test a bad case same seed with just slightly different
        //seeds
        let s = thread_rng().gen();
        let mut ra : Pcg32Unique = SeedableRng::from_seed(s);
        let mut rb : Pcg32Unique = SeedableRng::from_seed(s+1);
        assert!(ra.gen_ascii_chars().take(100).collect::<Vec<_>>() !=
                rb.gen_ascii_chars().take(100).collect::<Vec<_>>());
    }
}
