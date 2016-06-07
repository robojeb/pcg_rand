#![feature(convert,test)]

extern crate pcg_rand;
extern crate rand;
extern crate test;

use pcg_rand::{Pcg32, Pcg32Fast, Pcg32Basic, Pcg32L};
use test::Bencher;
use rand::{Rng, XorShiftRng, SeedableRng};

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
    let mut rng = Pcg32::from_seed(42);

    let mut x = vec![0; b.bytes as usize];

    b.iter(|| {
        rng.fill_bytes(x.as_mut_slice())
    })
}

#[bench]
fn pcg32L_next_u32(b: &mut Bencher) {
    let mut rng = Pcg32L::new_unseeded();

    b.iter(|| {
        rng.next_u32()
    })
}

#[bench]
fn pcg32L_fill_bytes(b: &mut Bencher) {
    b.bytes = 1024*1024;
    let mut rng = Pcg32L::from_seed([42, 41]);

    let mut x = vec![0; b.bytes as usize];

    b.iter(|| {
        rng.fill_bytes(x.as_mut_slice())
    })
}



#[bench]
fn pcg32basic_next_u32(b: &mut Bencher) {
    let mut rng = Pcg32Basic::from_seed([42, 41]);

    b.iter(|| {
        rng.next_u32()
    })
}

#[bench]
fn pcg32basic_fill_bytes(b: &mut Bencher) {
    b.bytes = 1024*1024;
    let mut rng = Pcg32Basic::from_seed([42,41]);

    let mut x = vec![0; b.bytes as usize];

    b.iter(|| {
        rng.fill_bytes(x.as_mut_slice())
    })
}

#[bench]
fn pcg32fast_next_u32(b: &mut Bencher) {
    let mut rng = Pcg32Fast::from_seed(42);

    b.iter(|| {
        rng.next_u32()
    })
}

#[bench]
fn pcg32fast_fill_bytes(b: &mut Bencher) {
    b.bytes = 1024*1024;
    let mut rng = Pcg32Fast::from_seed(42);

    let mut x = vec![0; b.bytes as usize];

    b.iter(|| {
        rng.fill_bytes(x.as_mut_slice())
    })
}

#[bench]
fn xorshift_next_u32(b: &mut Bencher) {
    let mut rng = XorShiftRng::new_unseeded();

    b.iter(|| {
        rng.next_u32()
    })
}

#[bench]
fn xorshift_fill_bytes(b: &mut Bencher) {
    b.bytes = 1024*1024;
    let mut rng = XorShiftRng::new_unseeded();

    let mut x = vec![0; b.bytes as usize];

    b.iter(|| {
        rng.fill_bytes(x.as_mut_slice())
    })
}
