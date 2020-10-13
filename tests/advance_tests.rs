
extern crate pcg_rand;
extern crate rand;

use pcg_rand::{Pcg32, Pcg64};
use rand::RngCore;

#[test]
fn pcg32_short_advance() {
    // Test that advancing a short distance is equal to going the long way round
    let mut ra: Pcg32 = Pcg32::new_unseeded();
    let mut rb: Pcg32 = Pcg32::new_unseeded();

    ra.advance(300);

    for _ in 0..300 {
        rb.next_u32();
    }

    assert_eq!(ra.next_u32(), rb.next_u32());
}

#[test]
fn pcg32_long_advance() {
    // Test that advancing a short distance is equal to going the long way round
    let mut ra: Pcg32 = Pcg32::new_unseeded();
    let mut rb: Pcg32 = Pcg32::new_unseeded();

    ra.advance(59032011);

    for _ in 0..59032011 {
        rb.next_u32();
    }

    assert_eq!(ra.next_u32(), rb.next_u32());
}

#[test]
fn pcg64_short_advance() {
    // Test that advancing a short distance is equal to going the long way round
    let mut ra: Pcg64 = Pcg64::new_unseeded();
    let mut rb: Pcg64 = Pcg64::new_unseeded();

    ra.advance(300);

    for _ in 0..300 {
        rb.next_u64();
    }

    assert_eq!(ra.next_u64(), rb.next_u64());
}

#[test]
fn pcg64_long_advance() {
    // Test that advancing a short distance is equal to going the long way round
    let mut ra: Pcg64 = Pcg64::new_unseeded();
    let mut rb: Pcg64 = Pcg64::new_unseeded();

    ra.advance(59032011);

    for _ in 0..59032011 {
        rb.next_u64();
    }

    assert_eq!(ra.next_u64(), rb.next_u64());
}

#[test]
fn pcg32_backstep() {
    let mut ra: Pcg32 = Pcg32::new_unseeded();
    let mut rb: Pcg32 = Pcg32::new_unseeded();

    // Test stepping forward and then backstepping by going all the way around
    ra.next_u32();
    ra.advance(u64::MAX);

    assert_eq!(ra.next_u32(), rb.next_u32());
}

#[test]
fn pcg64_backstep() {
    let mut ra: Pcg64 = Pcg64::new_unseeded();
    let mut rb: Pcg64 = Pcg64::new_unseeded();

    // Test stepping forward and then backstepping by going all the way around
    ra.next_u64();
    ra.advance(u128::MAX);

    assert_eq!(ra.next_u64(), rb.next_u64());
}