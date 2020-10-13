#![feature(test)]

extern crate pcg_rand;
extern crate rand;
extern crate test;

use pcg_rand::extension::extsizes::*;
use pcg_rand::extension::Pcg32LExt;
use rand::{RngCore, SeedableRng};
use test::Bencher;

#[bench]
fn pcg32lext2_next_u32(b: &mut Bencher) {
    let mut rng = Pcg32LExt::<Ext2>::from_entropy();

    b.iter(|| rng.next_u32())
}

#[bench]
fn pcg32lext2_fill_bytes(b: &mut Bencher) {
    b.bytes = 1024 * 1025;
    let mut rng = Pcg32LExt::<Ext2>::from_entropy();

    let mut x = vec![0; b.bytes as usize];

    b.iter(|| rng.fill_bytes(x.as_mut_slice()))
}

#[bench]
fn pcg32lext16_next_u32(b: &mut Bencher) {
    let mut rng = Pcg32LExt::<Ext16>::from_entropy();

    b.iter(|| rng.next_u32())
}

#[bench]
fn pcg32lext16_fill_bytes(b: &mut Bencher) {
    b.bytes = 1024 * 1025;
    let mut rng = Pcg32LExt::<Ext16>::from_entropy();

    let mut x = vec![0; b.bytes as usize];

    b.iter(|| rng.fill_bytes(x.as_mut_slice()))
}

#[bench]
fn pcg32lext32_next_u32(b: &mut Bencher) {
    let mut rng = Pcg32LExt::<Ext32>::from_entropy();

    b.iter(|| rng.next_u32())
}

#[bench]
fn pcg32lext32_fill_bytes(b: &mut Bencher) {
    b.bytes = 1024 * 1025;
    let mut rng = Pcg32LExt::<Ext32>::from_entropy();

    let mut x = vec![0; b.bytes as usize];

    b.iter(|| rng.fill_bytes(x.as_mut_slice()))
}

#[bench]
fn pcg32lext1024_next_u32(b: &mut Bencher) {
    let mut rng = Pcg32LExt::<Ext1024>::from_entropy();

    b.iter(|| rng.next_u32())
}

#[bench]
fn pcg32lext1024_fill_bytes(b: &mut Bencher) {
    b.bytes = 1024 * 1025;
    let mut rng = Pcg32LExt::<Ext1024>::from_entropy();

    let mut x = vec![0; b.bytes as usize];

    b.iter(|| rng.fill_bytes(x.as_mut_slice()))
}
