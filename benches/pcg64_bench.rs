#![feature(test)]

extern crate pcg_rand;
extern crate rand;
extern crate test;

use pcg_rand::{Pcg64, Pcg64Fast, Pcg64Unique};
use test::Bencher;
use rand::{Rng, RngCore, FromEntropy};

#[bench]
fn pcg64_next_u32(b: &mut Bencher) {
    let mut rng = Pcg64::from_entropy();

    b.iter(|| {
        rng.next_u32()
    })
}

#[bench]
fn pcg64_fill_bytes(b: &mut Bencher) {
    b.bytes = 1024*1024;
    let mut rng = Pcg64::from_entropy();

    let mut x = vec![0; b.bytes as usize];

    b.iter(|| {
        rng.fill_bytes(x.as_mut_slice())
    })
}

#[bench]
fn pcg64fast_next_u32(b: &mut Bencher) {
    let mut rng = Pcg64Fast::from_entropy();

    b.iter(|| {
        rng.next_u32()
    })
}

#[bench]
fn pcg64fast_fill_bytes(b: &mut Bencher) {
    b.bytes = 1024*1024;
    let mut rng = Pcg64Fast::from_entropy();

    let mut x = vec![0; b.bytes as usize];

    b.iter(|| {
        rng.fill_bytes(x.as_mut_slice())
    })
}


#[bench]
fn pcg64u_next_u32(b: &mut Bencher) {
    let mut rng = Pcg64Unique::from_entropy();

    b.iter(|| {
        rng.next_u32()
    })
}

#[bench]
fn pcg64u_fill_bytes(b: &mut Bencher) {
    b.bytes = 1024*1024;
    let mut rng = Pcg64Unique::from_entropy();

    let mut x = vec![0; b.bytes as usize];

    b.iter(|| {
        rng.fill_bytes(x.as_mut_slice())
    })
}