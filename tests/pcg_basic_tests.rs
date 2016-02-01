extern crate pcg_rand;
extern crate rand;

use rand::{Rng, SeedableRng};
use pcg_rand::Pcg32Basic;

#[test]
fn pcg_basic_unseeded() {
    let mut ra : Pcg32Basic = Pcg32Basic::new_unseeded();
    let mut rb : Pcg32Basic = Pcg32Basic::new_unseeded();
    assert_eq!(ra.gen_ascii_chars().take(100).collect::<Vec<_>>(),
               rb.gen_ascii_chars().take(100).collect::<Vec<_>>());
}

#[test]
fn pcg_basic_seed_match() {
    let mut ra : Pcg32Basic = SeedableRng::from_seed([11, 12]);
    let mut rb : Pcg32Basic = SeedableRng::from_seed([11, 12]);
    assert_eq!(ra.gen_ascii_chars().take(100).collect::<Vec<_>>(),
               rb.gen_ascii_chars().take(100).collect::<Vec<_>>());
}

#[test]
fn pcg_basic_seq_diff() {
    //Test a bad case same seed with just slightly different
    //sequences (They must be 2 apart because they get incremented to odd
    //numbers for generator properties)
    let mut ra : Pcg32Basic = SeedableRng::from_seed([11, 12]);
    let mut rb : Pcg32Basic = SeedableRng::from_seed([11, 14]);
    assert!(ra.gen_ascii_chars().take(100).collect::<Vec<_>>() !=
            rb.gen_ascii_chars().take(100).collect::<Vec<_>>());
}

#[test]
#[should_panic]
fn pcg_basic_seq_aliasing() {
    //Test a bad case same seed with just slightly different
    //sequences. These two end up being the same because 12 gets bumped
    //to 13 or the generator doesn't fill the entire range (needs a 1
    //in the lowest bit)
    //This is only a trait of PCGBasic not the other generators
    let mut ra : Pcg32Basic = SeedableRng::from_seed([11, 12]);
    let mut rb : Pcg32Basic = SeedableRng::from_seed([11, 13]);
    assert!(ra.gen_ascii_chars().take(100).collect::<Vec<_>>() !=
            rb.gen_ascii_chars().take(100).collect::<Vec<_>>());
}

#[test]
fn pcg_basic_seed_diff() {
    //Test a bad case same seed with just slightly different
    //seeds
    let mut ra : Pcg32Basic = SeedableRng::from_seed([11, 11]);
    let mut rb : Pcg32Basic = SeedableRng::from_seed([12, 11]);
    assert!(ra.gen_ascii_chars().take(100).collect::<Vec<_>>() !=
            rb.gen_ascii_chars().take(100).collect::<Vec<_>>());
}
