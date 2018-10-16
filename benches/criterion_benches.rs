#[macro_use]
extern crate criterion;
extern crate pcg_rand;
extern crate rand;

use criterion::*;
use pcg_rand::{extension::*, *};
use rand::{prng::Hc128Rng, FromEntropy, RngCore, XorShiftRng};

const KB: usize = 1024;
static BYTE_SIZES: [usize; 5] = [KB, 2 * KB, 4 * KB, 8 * KB, 16 * KB];

fn throughput_benchmark(c: &mut Criterion) {
    c.bench(
        "Throughput",
        ParameterizedBenchmark::new(
            "PCG32 Basic",
            |b, &&size| {
                let mut rng = Pcg32Basic::from_entropy();
                let mut x = vec![0u8; size];
                b.iter(|| rng.fill_bytes(&mut x))
            },
            &BYTE_SIZES,
        ).throughput(|&&size| Throughput::Bytes(size as u32))
        .with_function("PCG32", |b, &&size| {
            let mut rng = Pcg32::from_entropy();
            let mut x = vec![0u8; size];
            b.iter(|| rng.fill_bytes(&mut x))
        }).with_function("PCG32 Fast", |b, &&size| {
            let mut rng = Pcg32Fast::from_entropy();
            let mut x = vec![0u8; size];
            b.iter(|| rng.fill_bytes(&mut x))
        }).with_function("PCG64", |b, &&size| {
            let mut rng = Pcg64::from_entropy();
            let mut x = vec![0u8; size];
            b.iter(|| rng.fill_bytes(&mut x))
        }).with_function("PCG64 Fast", |b, &&size| {
            let mut rng = Pcg64Fast::from_entropy();
            let mut x = vec![0u8; size];
            b.iter(|| rng.fill_bytes(&mut x))
        }).with_function("Xorshift", |b, &&size| {
            let mut rng = XorShiftRng::from_entropy();
            let mut x = vec![0u8; size];
            b.iter(|| rng.fill_bytes(&mut x))
        }).with_function("Hc128", |b, &&size| {
            let mut rng = Hc128Rng::from_entropy();
            let mut x = vec![0u8; size];
            b.iter(|| rng.fill_bytes(&mut x))
        }),
    );

    c.bench(
        "Throughput 32-bit Long",
        ParameterizedBenchmark::new(
            "PCG32",
            |b, &&size| {
                let mut rng = Pcg32::from_entropy();
                let mut x = vec![0u8; size];
                b.iter(|| rng.fill_bytes(&mut x))
            },
            &BYTE_SIZES,
        ).throughput(|&&size| Throughput::Bytes(size as u32))
        .with_function("PCG32L", |b, &&size| {
            let mut rng = Pcg32L::from_entropy();
            let mut x = vec![0u8; size];
            b.iter(|| rng.fill_bytes(&mut x))
        }).with_function("PCG32L Fast", |b, &&size| {
            let mut rng = Pcg32LFast::from_entropy();
            let mut x = vec![0u8; size];
            b.iter(|| rng.fill_bytes(&mut x))
        }),
    );

    c.bench(
        "Throughput 32-bit Extended",
        ParameterizedBenchmark::new(
            "PCG32Ext2",
            |b, &&size| {
                let mut rng = Pcg32Ext::<Ext2>::from_entropy();
                let mut x = vec![0u8; size];
                b.iter(|| rng.fill_bytes(&mut x))
            },
            &BYTE_SIZES,
        ).throughput(|&&size| Throughput::Bytes(size as u32))
        .with_function("PCG32Ext32", |b, &&size| {
            let mut rng = Pcg32Ext::<Ext32>::from_entropy();
            let mut x = vec![0u8; size];
            b.iter(|| rng.fill_bytes(&mut x))
        }).with_function("PCG32Ext64", |b, &&size| {
            let mut rng = Pcg32Ext::<Ext64>::from_entropy();
            let mut x = vec![0u8; size];
            b.iter(|| rng.fill_bytes(&mut x))
        }).with_function("PCG32Ext128", |b, &&size| {
            let mut rng = Pcg32Ext::<Ext128>::from_entropy();
            let mut x = vec![0u8; size];
            b.iter(|| rng.fill_bytes(&mut x))
        }),
    );

    c.bench(
        "Throughput 64-bit Extended",
        ParameterizedBenchmark::new(
            "PCG64Ext2",
            |b, &&size| {
                let mut rng = Pcg64Ext::<Ext2>::from_entropy();
                let mut x = vec![0u8; size];
                b.iter(|| rng.fill_bytes(&mut x))
            },
            &BYTE_SIZES,
        ).throughput(|&&size| Throughput::Bytes(size as u32))
        .with_function("PCG64Ext32", |b, &&size| {
            let mut rng = Pcg64Ext::<Ext32>::from_entropy();
            let mut x = vec![0u8; size];
            b.iter(|| rng.fill_bytes(&mut x))
        }).with_function("PCG64Ext64", |b, &&size| {
            let mut rng = Pcg64Ext::<Ext64>::from_entropy();
            let mut x = vec![0u8; size];
            b.iter(|| rng.fill_bytes(&mut x))
        }).with_function("PCG64Ext128", |b, &&size| {
            let mut rng = Pcg64Ext::<Ext128>::from_entropy();
            let mut x = vec![0u8; size];
            b.iter(|| rng.fill_bytes(&mut x))
        }),
    );
}

fn generation_benchmarks(c: &mut Criterion) {
    c.bench(
        "next_u32",
        Benchmark::new("PCG32 Basic", |b| {
            let mut rng = Pcg32Basic::from_entropy();
            b.iter(|| rng.next_u32());
        }).with_function("PCG32 Fast", |b| {
            let mut rng = Pcg32Fast::from_entropy();
            b.iter(|| rng.next_u32());
        }).with_function("PCG32L", |b| {
            let mut rng = Pcg32L::from_entropy();
            b.iter(|| rng.next_u32());
        }).with_function("PCG64", |b| {
            let mut rng = Pcg64::from_entropy();
            b.iter(|| rng.next_u32());
        }).with_function("PCG64 Fast", |b| {
            let mut rng = Pcg64Fast::from_entropy();
            b.iter(|| rng.next_u32());
        }).with_function("Xorshift", |b| {
            let mut rng = XorShiftRng::from_entropy();
            b.iter(|| rng.next_u32());
        }).with_function("Hc128", |b| {
            let mut rng = Hc128Rng::from_entropy();
            b.iter(|| rng.next_u32());
        }),
    );

    c.bench(
        "next_u64",
        Benchmark::new("PCG32 Basic", |b| {
            let mut rng = Pcg32Basic::from_entropy();
            b.iter(|| rng.next_u64());
        }).with_function("PCG32 Fast", |b| {
            let mut rng = Pcg32Fast::from_entropy();
            b.iter(|| rng.next_u64());
        }).with_function("PCG32L", |b| {
            let mut rng = Pcg32L::from_entropy();
            b.iter(|| rng.next_u64());
        }).with_function("PCG64", |b| {
            let mut rng = Pcg64::from_entropy();
            b.iter(|| rng.next_u64());
        }).with_function("PCG64 Fast", |b| {
            let mut rng = Pcg64Fast::from_entropy();
            b.iter(|| rng.next_u64());
        }).with_function("Xorshift", |b| {
            let mut rng = XorShiftRng::from_entropy();
            b.iter(|| rng.next_u64());
        }).with_function("Hc128", |b| {
            let mut rng = Hc128Rng::from_entropy();
            b.iter(|| rng.next_u64());
        }),
    );
}

criterion_group!(benches, throughput_benchmark, generation_benchmarks);
criterion_main!(benches);
