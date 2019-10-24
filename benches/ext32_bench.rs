#![feature(test)]

extern crate pcg_rand;
extern crate rand;
extern crate test;

use pcg_rand::extension::extsizes::*;
use pcg_rand::extension::Pcg32Ext;
use rand::{SeedableRng, RngCore};
use test::Bencher;

#[bench]
fn pcg32ext2_next_u32(b: &mut Bencher) {
    let mut rng = Pcg32Ext::<Ext2>::from_entropy();

    b.iter(|| rng.next_u32())
}

#[bench]
fn pcg32ext2_fill_bytes(b: &mut Bencher) {
    b.bytes = 1024 * 1025;
    let mut rng = Pcg32Ext::<Ext2>::from_entropy();

    let mut x = vec![0; b.bytes as usize];

    b.iter(|| rng.fill_bytes(x.as_mut_slice()))
}

#[bench]
fn pcg32ext16_next_u32(b: &mut Bencher) {
    let mut rng = Pcg32Ext::<Ext16>::from_entropy();

    b.iter(|| rng.next_u32())
}

#[bench]
fn pcg32ext16_fill_bytes(b: &mut Bencher) {
    b.bytes = 1024 * 1025;
    let mut rng = Pcg32Ext::<Ext16>::from_entropy();

    let mut x = vec![0; b.bytes as usize];

    b.iter(|| rng.fill_bytes(x.as_mut_slice()))
}

#[bench]
fn pcg32ext32_next_u32(b: &mut Bencher) {
    let mut rng = Pcg32Ext::<Ext32>::from_entropy();

    b.iter(|| rng.next_u32())
}

#[bench]
fn pcg32ext32_fill_bytes(b: &mut Bencher) {
    b.bytes = 1024 * 1025;
    let mut rng = Pcg32Ext::<Ext32>::from_entropy();

    let mut x = vec![0; b.bytes as usize];

    b.iter(|| rng.fill_bytes(x.as_mut_slice()))
}

#[bench]
fn pcg32ext1024_next_u32(b: &mut Bencher) {
    let mut rng = Pcg32Ext::<Ext1024>::from_entropy();

    b.iter(|| rng.next_u32())
}

#[bench]
fn pcg32ext1024_fill_bytes(b: &mut Bencher) {
    b.bytes = 1024 * 1025;
    let mut rng = Pcg32Ext::<Ext1024>::from_entropy();

    let mut x = vec![0; b.bytes as usize];

    b.iter(|| rng.fill_bytes(x.as_mut_slice()))
}
