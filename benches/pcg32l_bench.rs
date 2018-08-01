#![feature(test)]

extern crate pcg_rand;
extern crate rand;
extern crate test;

use pcg_rand::{Pcg32L, Pcg32LFast};
use test::Bencher;
use rand::{RngCore, FromEntropy};

#[bench]
fn pcg32l_next_u32(b: &mut Bencher) {
    let mut rng = Pcg32L::from_entropy();

    b.iter(|| {
        rng.next_u32()
    })
}

#[bench]
fn pcg32l_fill_bytes(b: &mut Bencher) {
    b.bytes = 1024*1024;
    let mut rng = Pcg32L::from_entropy();

    let mut x = vec![0; b.bytes as usize];

    b.iter(|| {
        rng.fill_bytes(x.as_mut_slice())
    })
}

#[bench]
fn pcg32lfast_next_u32(b: &mut Bencher) {
    let mut rng = Pcg32LFast::from_entropy();

    b.iter(|| {
        rng.next_u32()
    })
}

#[bench]
fn pcg32lfast_fill_bytes(b: &mut Bencher) {
    b.bytes = 1024*1024;
    let mut rng = Pcg32LFast::from_entropy();

    let mut x = vec![0; b.bytes as usize];

    b.iter(|| {
        rng.fill_bytes(x.as_mut_slice())
    })
}