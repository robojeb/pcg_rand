extern crate pcg_rand;
extern crate rand;

use pcg_rand::seeds::PcgSeeder;
use pcg_rand::Pcg32Unique;
use rand::{distributions::Alphanumeric, thread_rng, Rng, SeedableRng};

const NUM_TESTS: usize = 1000;

#[test]
#[should_panic]
fn pcg32_unique_unseeded() {
    let mut ra: Pcg32Unique = Pcg32Unique::new_unseeded();
    let mut rb: Pcg32Unique = Pcg32Unique::new_unseeded();
    //Because these are unique these should not match
    assert!(
        ra.sample_iter(&Alphanumeric).take(100).collect::<Vec<_>>() !=
        rb.sample_iter(&Alphanumeric).take(100).collect::<Vec<_>>()
    );
}

#[test]
#[should_panic]
fn pcg32_unique_seed_match() {
    for _ in 0..NUM_TESTS {
        let s = PcgSeeder::seed(thread_rng().gen());
        let mut ra: Pcg32Unique = SeedableRng::from_seed(s.clone());
        let mut rb: Pcg32Unique = SeedableRng::from_seed(s);
        //Because these are unique these should not match
        assert!(
            ra.sample_iter(&Alphanumeric).take(100).collect::<Vec<_>>() !=
            rb.sample_iter(&Alphanumeric).take(100).collect::<Vec<_>>()
        );
    }
}

#[test]
fn pcg32_unique_seed_diff() {
    for _ in 0..NUM_TESTS {
        //Test a bad case same seed with just slightly different
        //seeds
        let seed: u64 = thread_rng().gen();
        let s1 = PcgSeeder::seed(seed);
        let s2 = PcgSeeder::seed(seed + 1);
        let mut ra: Pcg32Unique = SeedableRng::from_seed(s1);
        let mut rb: Pcg32Unique = SeedableRng::from_seed(s2);
        assert!(
            ra.sample_iter(&Alphanumeric).take(100).collect::<Vec<_>>()
                != rb.sample_iter(&Alphanumeric).take(100).collect::<Vec<_>>()
        );
    }
}
