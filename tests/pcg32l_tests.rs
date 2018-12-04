#![cfg(feature = "u128")]
extern crate pcg_rand;
extern crate rand;

use pcg_rand::seeds::PcgSeeder;
use pcg_rand::Pcg32L;
use rand::{distributions::Alphanumeric, thread_rng, Rng, SeedableRng};

const NUM_TESTS: usize = 1000;

#[test]
fn Pcg32L_unseeded() {
    let mut ra: Pcg32L = Pcg32L::new_unseeded();
    let mut rb: Pcg32L = Pcg32L::new_unseeded();
    assert_eq!(
        ra.sample_iter(&Alphanumeric).take(100).collect::<Vec<_>>(),
        rb.sample_iter(&Alphanumeric).take(100).collect::<Vec<_>>()
    );
}

#[test]
fn Pcg32L_seed_match() {
    for _ in 0..NUM_TESTS {
        let seed: u64 = thread_rng().gen();
        let seq: u64 = thread_rng().gen();
        let s = PcgSeeder::seed_with_stream(seed as u128, seq as u128);
        let mut ra: Pcg32L = SeedableRng::from_seed(s.clone());
        let mut rb: Pcg32L = SeedableRng::from_seed(s);
        assert_eq!(
            ra.sample_iter(&Alphanumeric).take(100).collect::<Vec<_>>(),
            rb.sample_iter(&Alphanumeric).take(100).collect::<Vec<_>>()
        );
    }
}

#[test]
fn Pcg32L_seq_diff() {
    for _ in 0..NUM_TESTS {
        //Test a bad case same seed with just slightly different
        //sequences. Because sequences have to be odd only sequences that are 2 apart
        //are for sure going to be different.
        let seed: u64 = thread_rng().gen();
        let seq: u64 = thread_rng().gen();
        let mut ra: Pcg32L =
            Pcg32L::from_seed(PcgSeeder::seed_with_stream(seed as u128, seq as u128));
        let mut rb: Pcg32L =
            Pcg32L::from_seed(PcgSeeder::seed_with_stream(seed as u128, (seq + 2) as u128));
        assert!(
            ra.sample_iter(&Alphanumeric).take(100).collect::<Vec<_>>()
                != rb.sample_iter(&Alphanumeric).take(100).collect::<Vec<_>>()
        );
    }
}

#[test]
fn Pcg32L_seed_diff() {
    for _ in 0..NUM_TESTS {
        //Test a bad case same seed with just slightly different
        //seeds
        let seed: u64 = thread_rng().gen();
        let seq: u64 = thread_rng().gen();
        let mut ra: Pcg32L =
            Pcg32L::from_seed(PcgSeeder::seed_with_stream(seed as u128, seq as u128));
        let mut rb: Pcg32L =
            Pcg32L::from_seed(PcgSeeder::seed_with_stream((seed + 1) as u128, seq as u128));
        assert!(
            ra.sample_iter(&Alphanumeric).take(100).collect::<Vec<_>>()
                != rb.sample_iter(&Alphanumeric).take(100).collect::<Vec<_>>()
        );
    }
}
