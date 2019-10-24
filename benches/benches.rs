#![feature(test)]

extern crate pcg_rand;
extern crate rand;
extern crate rand_core;
extern crate test;

use pcg_rand::Pcg32Basic;
use rand::{SeedableRng, rngs::{StdRng}};
use rand_core::RngCore;
use test::Bencher;

#[bench]
fn pcg32basic_next_u32(b: &mut Bencher) {
    let mut rng = Pcg32Basic::from_entropy();

    b.iter(|| rng.next_u32())
}

#[bench]
fn pcg32basic_fill_bytes(b: &mut Bencher) {
    b.bytes = 1024 * 1024;
    let mut rng = Pcg32Basic::from_entropy();

    let mut x = vec![0; b.bytes as usize];

    b.iter(|| rng.fill_bytes(x.as_mut_slice()))
}

#[bench]
fn stdrng_next_u32(b: &mut Bencher) {
    let mut rng = StdRng::from_entropy();

    b.iter(|| rng.next_u32())
}

#[bench]
fn stdrng_fill_bytes(b: &mut Bencher) {
    b.bytes = 1024 * 1024;
    let mut rng = StdRng::from_entropy();

    let mut x = vec![0; b.bytes as usize];

    b.iter(|| rng.fill_bytes(x.as_mut_slice()))
}

// #[bench]
// fn smallrng_next_u32(b: &mut Bencher) {
//     let mut rng = SmallRng::from_entropy();

//     b.iter(|| rng.next_u32())
// }

// #[bench]
// fn smallrng_fill_bytes(b: &mut Bencher) {
//     b.bytes = 1024 * 1024;
//     let mut rng = SmallRng::from_entropy();

//     let mut x = vec![0; b.bytes as usize];

//     b.iter(|| rng.fill_bytes(x.as_mut_slice()))
// }
