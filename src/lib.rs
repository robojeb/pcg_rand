/*
 * PCG Random Number Generation for Rust
 *
 * Copyright 2015 John Brooks <robojeb@robojeb.xyz>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 * This work is derived from the implementation PCG RNG for C++ by
 * Melissa O'Neill.
 *
 * For additional information about the PCG random number generation scheme,
 * including its license and other licensing options, visit
 *
 *     http://www.pcg-random.org
 */

//! An implementation of the PCG random family of random number generators.
//! Details about the PCG generators can be found at [pcg-random.org](http://pcg-random.org)
//!
//! Currently this library provides several PCG generators:
//!
//! * `Pcg32` : 64bit LCG with an xorshift and right rotation applied to the output. To improve
//!   security only 32bits of the state are reported as output.
//! * `Pcg32Onseq` : Same as `Pcg32` but with a fixed sequence. Useful if you don't care about having
//!   multiple streams of random numbers from the same seed.
//! * `Pcg32Unique` : Same as `Pcg32` but the sequence is set by the memory location of the RNG
//!   This means that multiple `Pcg32_unique` with the same seed will produce different sequences
//!   of numbers.
//!
//!
//! # Usage
//!
//! This crate is [on crates.io](https://crates.io/crates/pcg_rand) and can be used by
//! adding the `pcg_rand` crate to your projects Cargo.toml
//!
//! ```toml
//! [dependencies]
//! pcg_rand = "0.3.1"
//! ```
//! #Typename Nomenclature
//! This library attempts to simplify using the PCG generators by defining easy
//! types for use. The following attempts to help you decode these typenames
//!
//! Consider the example `OneseqXshRr6432`. This consists of 4 major parts.
//!
//! 1. First is the sequence type
//! 1. Second is the permutation function
//! 1. Third is the state size in bits
//! 1. Fourth is the output size in bits
//!
//! ## Sequence types
//!
//! This library provides the following sequence types
//!
//! * `Setseq`: This is a settable stream. The random number stream can be set manually.
//! * `Unique`: This is a unique stream. Each instance of this type will be given a unique stream
//!   that cannot be modified.
//! * `Oneseq`: This is one fixed random sequence. It is hardcoded into the library and should be
//!   good enough to give good "randomness".
//! * `Mcg`: This has no random sequence it degenerates the internal LCG into a MCG. This is for
//!   speed.
//!
//! ## Permutation functions
//!
//! There are many possible permuation functions that this library can implement. Many of them are
//! composed of several indiviual components. The components that are used are:
//!
//! * `Xsh`: Refers to a High Xorshift function.
//! * `Rr`: Refers to a random rotation. Randomly rotates based on entropy from the state.
//! * `Rs`: Refers to a random shift. Randomly shifts based on entropy from the state.


extern crate rand;

use rand::{Rng, Rand, SeedableRng};

use std::num::Wrapping;

mod stream;
mod multiplier;
mod outputmix;

use stream::{Stream, OneSeqStream, SpecificSeqStream, UniqueSeqStream, NoSeqStream};
use multiplier::{Multiplier, DefaultMultiplier, McgMultiplier};
use outputmix::{OutputMixin, XshRsMixin, XshRrMixin};

use std::marker::PhantomData;

/// A generic PCG structure.
///
/// This structure allows the building of many types of PCG generators by using various
/// Mixins for both the stream, multiplier, and permutation function.
pub struct PcgEngine<Itype, Xtype,
    StreamMix : Stream<Itype>,
    MulMix : Multiplier<Itype>,
    OutMix : OutputMixin<Itype, Xtype>>
{
    state      : Itype,
    stream_mix : StreamMix,
    mul_mix    : MulMix,
    out_mix    : OutMix,
    phantom    : PhantomData<Xtype>
}

macro_rules! build_basic_pcg {
    ( $($name:ident, $itype:ty, $xtype:ty, $seq:ident, $mul:ident, $out:ident);* ) => (
        $(
            /// A helper definition for a PcgEngine with parameters defined in the name
            pub type $name = PcgEngine<$itype, $xtype, $seq, $mul, $out>;

            impl $name {
                /// Creates a new unseeded PCG
                /// This will have state 0 and a sequence based on its sequence type
                pub fn new_unseeded() -> $name {
                    PcgEngine{
                        state      : 0,
                        stream_mix : $seq::new(),
                        mul_mix    : $mul,
                        out_mix    : $out,
                        phantom    : PhantomData::<$xtype>,
                    }
                }
            }

            impl Rng for $name {
                #[inline]
                fn next_u32(&mut self) -> u32 {
                    let oldstate = Wrapping(self.state.clone());
                    let mul = Wrapping::<$itype>(self.mul_mix.multiplier());
                    let inc = Wrapping::<$itype>(self.stream_mix.increment());
                    self.state = (oldstate * mul + inc).0;

                    self.out_mix.output(oldstate.0)
                }
            }

            impl Rand for $name {
                fn rand<R: Rng>(other: &mut R) -> $name {
                    PcgEngine{
                        state      : other.gen(),
                        stream_mix : $seq::new(),
                        mul_mix    : $mul,
                        out_mix    : $out,
                        phantom    : PhantomData::<$xtype>,
                    }
                }
            }

            impl SeedableRng<$itype> for $name {
                fn reseed(&mut self, seed: $itype) {
                    self.state = seed;
                }

                fn from_seed(seed: $itype) -> $name {
                    PcgEngine{
                        state      : seed,
                        stream_mix : $seq::new(),
                        mul_mix    : $mul,
                        out_mix    : $out,
                        phantom    : PhantomData::<$xtype>,
                    }
                }
            }
        )*
    )
}

macro_rules! build_sequence_pcg {
    ( $($name:ident, $itype:ty, $xtype:ty, $seq:ident, $mul:ident, $out:ident);* ) => (
        $(
            /// A helper definition for a PcgEngine with parameters defined in the name
            pub type $name = PcgEngine<$itype, $xtype, $seq<$itype>, $mul, $out>;

            impl $name {
                /// Creates a new unseeded PCG
                /// This will have state 0 and sequence 1
                pub fn new_unseeded() -> $name {
                    PcgEngine{
                        state      : 0,
                        stream_mix : $seq::<$itype>::new(),
                        mul_mix    : $mul,
                        out_mix    : $out,
                        phantom    : PhantomData::<$xtype>,
                    }
                }

                pub fn set_stream(&mut self, stream : $itype) {
                    self.stream_mix.set_stream(stream);
                }
            }

            impl Rng for $name {
                #[inline]
                fn next_u32(&mut self) -> u32 {
                    let oldstate = Wrapping(self.state.clone());
                    let mul = Wrapping::<$itype>(self.mul_mix.multiplier());
                    let inc = Wrapping::<$itype>(self.stream_mix.increment());
                    self.state = (oldstate * mul + inc).0;

                    self.out_mix.output(oldstate.0)
                }
            }

            impl Rand for $name {
                fn rand<R: Rng>(other: &mut R) -> $name {
                    let mut stream_mix = $seq::<$itype>::new();
                    stream_mix.set_stream(other.gen());
                    PcgEngine{
                        state      : other.gen(),
                        stream_mix : stream_mix,
                        mul_mix    : $mul,
                        out_mix    : $out,
                        phantom    : PhantomData::<$xtype>,
                    }
                }
            }

            impl SeedableRng<[$itype; 2]> for $name {
                fn reseed(&mut self, seed: [$itype; 2]) {
                    self.state = seed[0];
                    self.stream_mix.set_stream(seed[0]);
                }

                fn from_seed(seed: [$itype; 2]) -> $name {
                    let mut stream_mix = $seq::<$itype>::new();
                    stream_mix.set_stream(seed[1]);
                    PcgEngine{
                        state      : seed[0],
                        stream_mix : stream_mix,
                        mul_mix    : $mul,
                        out_mix    : $out,
                        phantom    : PhantomData::<$xtype>,
                    }
                }
            }

            impl SeedableRng<$itype> for $name {
                fn reseed(&mut self, seed: $itype) {
                    self.state = seed;
                }

                fn from_seed(seed: $itype) -> $name {
                    PcgEngine{
                        state      : seed,
                        stream_mix : $seq::<$itype>::new(),
                        mul_mix    : $mul,
                        out_mix    : $out,
                        phantom    : PhantomData::<$xtype>,
                    }
                }
            }
        )*
    )
}

build_basic_pcg!(
    OneseqXshRs6432, u64, u32, OneSeqStream, DefaultMultiplier, XshRsMixin;
    UniqueXshRs6432, u64, u32, UniqueSeqStream, DefaultMultiplier, XshRsMixin;
    OneseqXshRr6432, u64, u32, OneSeqStream, DefaultMultiplier, XshRrMixin;
    UniqueXshRr6432, u64, u32, UniqueSeqStream, DefaultMultiplier, XshRrMixin;
    McgXshRs6432,    u64, u32, NoSeqStream, McgMultiplier, XshRsMixin;
    McgXshRr6432,    u64, u32, NoSeqStream, McgMultiplier, XshRrMixin
);

build_sequence_pcg!(
    SetseqXshRs6432, u64, u32, SpecificSeqStream, DefaultMultiplier, XshRsMixin;
    SetseqXshRr6432, u64, u32, SpecificSeqStream, DefaultMultiplier, XshRrMixin
);

/// A helper definition for a simple 32bit PCG which can have multiple random streams
pub type Pcg32       = SetseqXshRr6432;
/// A helper definition for a 32bit PCG which hase a fixed good random stream
pub type Pcg32Oneseq = OneseqXshRr6432;
/// A helper definition for a 32bit PCG which has a unique random stream for each instance
pub type Pcg32Unique = UniqueXshRr6432;
/// A helper definition for a 32bit PCG which is fast but may lack statistical quality.
///
/// This generator sacrifices quality for speed by utilizing a Multiplicative Congruential
/// generator instead of a LCG. Additionally it uses a simpler permutation function so that the
/// compiler can optimize and reduce the number of operations.
pub type Pcg32Fast = McgXshRs6432;

/*
 * The simple C minimal implementation of PCG32
 */

///A low overhead very simple PCG impementation
///
///This is mostly useful for demonstrating how PCG works.
///If you want better statistical performance you should use one of the predefined types like
///`Pcg32`.
pub struct Pcg32Basic {
    state : u64,
    inc   : u64,
}

impl Pcg32Basic {
    pub fn new_unseeded() -> Pcg32Basic {
        Pcg32Basic{
            state : 0,
            inc : 1,
        }
    }
}

//Pcg32Basic is an rng
impl Rng for Pcg32Basic {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        let oldstate = Wrapping(self.state);
        //Update the state as an lcg
        self.state = (oldstate * Wrapping(6364136223846793005u64) + Wrapping(self.inc | 1)).0;

        //Prepare the permutation on the output
        let xorshifted : u32 = (((oldstate >> 18usize) ^ oldstate) >> 27usize).0 as u32;
        let rot : u32 = (oldstate >> 59usize).0 as u32;

        //Produce the permuted output
        (xorshifted >> rot) | (xorshifted << ((-(rot as i32)) & 31))
    }
}

//Allow seeding of Pcg32Basic
impl SeedableRng<[u64; 2]> for Pcg32Basic {
    fn reseed(&mut self, seed: [u64; 2]) {
        self.state = seed[0];
        self.inc   = seed[1];
    }

    fn from_seed(seed: [u64; 2]) -> Pcg32Basic {
        Pcg32Basic {
            state : seed[0],
            inc   : seed[1],
        }
    }
}

//Pcg32Basic can be randomly initialized with system entropy (or any other RNG)
impl Rand for Pcg32Basic {
    fn rand<R: Rng>(other: &mut R) -> Pcg32Basic {
        Pcg32Basic{
            state : other.gen(),
            inc   : other.gen(),
        }
    }
}
