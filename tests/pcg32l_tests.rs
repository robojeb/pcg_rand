extern crate pcg_rand;
extern crate rand;

use rand::{Rng, SeedableRng, thread_rng};
use pcg_rand::Pcg32L;

const NUM_TESTS : usize = 1000;

#[test]
fn pcg32l_unseeded() {
    let mut ra : Pcg32L = Pcg32L::new_unseeded();
    let mut rb : Pcg32L = Pcg32L::new_unseeded();
    assert_eq!(ra.gen_ascii_chars().take(100).collect::<Vec<_>>(),
               rb.gen_ascii_chars().take(100).collect::<Vec<_>>());
}

#[test]
fn pcg32l_seed_match() {
    for _ in 0..NUM_TESTS {
        let s : [u64;4] = thread_rng().gen();
        let mut ra : Pcg32L = SeedableRng::from_seed(s);
        let mut rb : Pcg32L = SeedableRng::from_seed(s);
        assert_eq!(ra.gen_ascii_chars().take(100).collect::<Vec<_>>(),
                   rb.gen_ascii_chars().take(100).collect::<Vec<_>>());
    }
}

#[test]
fn pcg32l_seq_diff() {
    for _ in 0..NUM_TESTS {
        //Test a bad case same seed with just slightly different
        //sequences. Because sequences have to be odd only sequences that are 2 apart 
        //are for sure going to be different.
        let mut s : [u64;4] = thread_rng().gen();
        let mut ra : Pcg32L = SeedableRng::from_seed(s);
        s[1] = s[1]+2;
        let mut rb : Pcg32L = SeedableRng::from_seed(s);
        assert!(ra.gen_ascii_chars().take(100).collect::<Vec<_>>() !=
                rb.gen_ascii_chars().take(100).collect::<Vec<_>>());
    }
}

#[test]
fn pcg32l_seed_diff() {
    for _ in 0..NUM_TESTS {
        //Test a bad case same seed with just slightly different
        //seeds
        let mut s : [u64;4] = thread_rng().gen();
        let mut ra : Pcg32L = SeedableRng::from_seed(s);
        s[0] = s[0]+1;
        let mut rb : Pcg32L = SeedableRng::from_seed(s);
        assert!(ra.gen_ascii_chars().take(100).collect::<Vec<_>>() !=
                rb.gen_ascii_chars().take(100).collect::<Vec<_>>());
    }
}
