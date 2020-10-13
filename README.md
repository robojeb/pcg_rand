pcg_rand
========

[![Crates.io Version](https://img.shields.io/crates/v/pcg_rand.svg)](https://crates.io/crates/pcg_rand)
[![](https://docs.rs/pcg_rand/badge.svg)](https://docs.rs/pcg_rand/)
[![Build Status](https://travis-ci.com/robojeb/pcg_rand.svg?branch=master)](https://travis-ci.com/robojeb/pcg_rand)
[![License](https://img.shields.io/crates/l/pcg_rand.svg)](https://github.com/robojeb/pcg_rand/blob/master/LICENSE)
[![](https://img.shields.io/badge/rust-1.32%2B-blue.svg)](https://github.com/robojeb/pcg_rand)

To use this library add the following to your `Cargo.toml`

```
pcg_rand = "0.12.0"
```

PCG stands for Permuted Congruential generators. They are a simple family of
random number generators which use the much denounced Linear Congruential
Generator as a base. To overcome the well known limitations of the LCG Generator
the PCG family utilizes permutation functions to permute the output. More
information about PCG can be found [here](http://pcg-random.org).

Current Status
--------------
This library currently provides 32 and 64 bit generators. 
It provides the major "stream" types, including the unique stream which is determined
by their current location in memory. 

Future Work
-----------

* Implement seeking for the generators

Changes
-------
 * (6/8/2016): Added support for extended generators. This implementation
 is based on my understanding of how the extension is specified in the paper. 
 I would love a code review on the implementation. I don't think that this 
 particular implementation is the same as the implementation in the C++ version
 of PCG.
 * (6/7/2016): Added back some of the macros which got removed in 0.5.0
 This is in an effort to improve some performance. Hopefully associated constants
 can help us remove these again, but for now removing the PcgConsts trait gives
 some extra performance and reduces indirection. It also means that it is now
 easier to implement new streams and multipliers because you aren't bound
 to using the three things in PcgConsts.
 * (7/31/2018): Remembered to update the README (including changelog). Migrated
 to using the `rand: 0.5` crate which involves new methods. Most of the functions
 are now not based on macro's but instead are fully generic and utilize the 
 `num-traits` wrapping traits as needed. Additionally the crate now has support
 for using the native `u128` since that is stable in Rust now. This pushes the 
 minimum Rust version to 1.26. 
 * (8/2/2018): Provided better defaults for the `new_unseeded` function so that
 it can be useful for testing. Improved the docs to suggest seeding and updated
 the documentation examples to use `from_entropy` instead of `new_unseeded` to 
 promote good use of the generators. 
* (10/15/2018): Added cargo features for `u128` and experimental `serde1` 
 support. Also fixed an issue in `0.9.2` which caused it to be nightly only 
 (oops). Started migrating benchmarks to critereon. 
* (12/4/2018): Upgraded to `rand 0.6` and `rand_core 0.3`. Fixed an issue where 
 tests wouldn't run if there was no `u128` support. Serde support is still 
 highly experimental. 
* (12/12/2018): Bump to 0.10.1 to include the CI build badge on crates.io
* (10/24/2019): Bump to 0.11.0 to use `rand 0.7` and `rand_core 0.5`
* (10/28/2019): Bump to 0.11.1. Added the ability to make generators output a 
 a permutation of their entire internal state. 
* (10/02/2020): Update to 0.12.0. Added the ability to extract the internal 
 state of the generator. Moved the examples from `bin` to `examples` for 
 better discovery. Changed the serialization derivation, it only makes sense 
 to deserialize into a `SetSeq` type so it is the only one which supports
 deserialization. The serialization step writes out the internal state as
 an array and does not record any information about the type of generator. 
 * (10/*/2020): Added the DXsM output and made it the default (this is the 
 specified intention of the C++ library author). Re-worked serialization to 
 create a definition structure which contains all the information needed to
 verify that the PCG you are serializing into/out-of is compatible with the 
 stored information. This can be disabled with the `no_deserialize_verify` 
 feature. Also re-worked seeds to be an array of bits with a helper function
 which lets you define the sequence and increment more naturally. 