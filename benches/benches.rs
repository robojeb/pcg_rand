#![feature(convert,test)]

extern crate pcg_rand;
extern crate rand;
extern crate extprim;
extern crate test;

use pcg_rand::{Pcg32, Pcg32Fast, Pcg32Basic, Pcg32L};
use pcg_rand::extension::Pcg32Ext;
use pcg_rand::extension::extsizes::*;
use test::Bencher;
use rand::{Rng, XorShiftRng, SeedableRng};
use extprim::u128::u128;


#[bench]
fn u128_mul(b: &mut Bencher) {
   let mut a = u128::from_parts(23,58);
   let c = u128::from_parts(44,85);

   b.iter(|| {
       a = a.wrapping_mul(c);
   })
}

#[bench]
fn u128_shr(b: &mut Bencher) {
    let mut a = u128::from_parts(288818288,2888490028);

    b.iter(|| { a = a.wrapping_shr(16)});
}

#[bench]
fn u128_add(b: &mut Bencher) {
    let mut x = u128::from_parts(188281919932,18848482929);
    let y = u128::from_parts(882327887118,848898198399);

    b.iter(||{x = x.wrapping_add(y)});
}

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

#[bench]
fn pcg32ext32_fill_bytes(b: &mut Bencher) {
    b.bytes = 1024*1025;
    let mut rng = Pcg32Ext<Ext32>::new_unseeded();

    let mut x = vec![0; b.bytes as usize];

    b.iter(|| {
        rng.fill_bytes(x.as_mut_slice())
    })
}
