#![feature(test)]

extern crate pcg_rand;
extern crate rand;
extern crate test;

use pcg_rand::{Pcg32, Pcg32Fast};
use test::Bencher;
use rand::{Rng,SeedableRng};

#[bench]
fn pcg32_next_u32(b: &mut Bencher) {
    let mut rng = Pcg32::new_unseeded();

    b.iter(|| {
        rng.next_u32()
    })
}

#[bench]
fn pcg32_fill_bytes(b: &mut Bencher) {
    b.bytes = 1024*1024;
    let mut rng = Pcg32::new_unseeded();

    let mut x = vec![0; b.bytes as usize];

    b.iter(|| {
        rng.fill_bytes(x.as_mut_slice())
    })
}

#[bench]
fn pcg32fast_next_u32(b: &mut Bencher) {
    let mut rng = Pcg32Fast::new_unseeded();

    b.iter(|| {
        rng.next_u32()
    })
}

#[bench]
fn pcg32fast_fill_bytes(b: &mut Bencher) {
    b.bytes = 1024*1024;
    let mut rng = Pcg32Fast::new_unseeded();

    let mut x = vec![0; b.bytes as usize];

    b.iter(|| {
        rng.fill_bytes(x.as_mut_slice())
    })
}