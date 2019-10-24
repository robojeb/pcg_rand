#![feature(test)]

extern crate pcg_rand;
extern crate rand;
extern crate test;

use pcg_rand::{Pcg32, Pcg32Fast, Pcg32Unique};
use rand::{SeedableRng, RngCore};
use test::Bencher;

#[bench]
fn pcg32_next_u32(b: &mut Bencher) {
    let mut rng = Pcg32::from_entropy();

    b.iter(|| rng.next_u32())
}

#[bench]
fn pcg32_fill_bytes(b: &mut Bencher) {
    b.bytes = 1024 * 1024;
    let mut rng = Pcg32::from_entropy();

    let mut x = vec![0; b.bytes as usize];

    b.iter(|| rng.fill_bytes(x.as_mut_slice()))
}

#[bench]
fn pcg32fast_next_u32(b: &mut Bencher) {
    let mut rng = Pcg32Fast::from_entropy();

    b.iter(|| rng.next_u32())
}

#[bench]
fn pcg32fast_fill_bytes(b: &mut Bencher) {
    b.bytes = 1024 * 1024;
    let mut rng = Pcg32Fast::from_entropy();

    let mut x = vec![0; b.bytes as usize];

    b.iter(|| rng.fill_bytes(x.as_mut_slice()))
}

#[bench]
fn pcg32u_next_u32(b: &mut Bencher) {
    let mut rng = Pcg32Unique::from_entropy();

    b.iter(|| rng.next_u32())
}

#[bench]
fn pcg32u_fill_bytes(b: &mut Bencher) {
    b.bytes = 1024 * 1024;
    let mut rng = Pcg32Unique::from_entropy();

    let mut x = vec![0; b.bytes as usize];

    b.iter(|| rng.fill_bytes(x.as_mut_slice()))
}
