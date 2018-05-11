#![feature(test)]

extern crate pcg_rand;
extern crate rand;
extern crate test;

use pcg_rand::Pcg32Basic;
use test::Bencher;
use rand::{Rng, XorShiftRng, SeedableRng};


// #[bench]
// fn u128_mul(b: &mut Bencher) {
//    let mut a = 424275113695319687226; //u128::from_parts(23,58);
//    let c = 811656739243220271189; //u128::from_parts(44,85);

//    b.iter(|| {
//        a = a.wrapping_mul(c);
//    })
// }

// #[bench]
// fn u128_shr(b: &mut Bencher) {
//     let mut a = 5327757042542938509869243436; //u128::from_parts(288818288,2888490028);

//     b.iter(|| { a = a.wrapping_shr(16)});
// }

// #[bench]
// fn u128_add(b: &mut Bencher) {
//     let mut x = 5327757042542938509869243436; // NOT: u128::from_parts(188281919932,18848482929);
//     let y = 5327757042542938509869243436; // NOT: u128::from_parts(882327887118,848898198399);

//     b.iter(||{x = x.wrapping_add(y)});
// }

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


