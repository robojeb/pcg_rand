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
//!   of numbers. *NOTE*: This means that you may not get consistant results across runs of your
//!   program. If the memory location of your PCG moves for any reason such as the state of the
//!   allocator being different you will get a different stream of numbers.
//!
//!
//! # Usage
//!
//! This crate is [on crates.io](https://crates.io/crates/pcg_rand) and can be used by
//! adding the `pcg_rand` crate to your projects Cargo.toml
//!
//! ```toml
//! [dependencies]
//! pcg_rand = "0.5.0"
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
//!


extern crate rand;
extern crate extprim;

use rand::{Rng, Rand, SeedableRng};

use std::num::Wrapping;

use extprim::u128::u128;

mod stream;
mod multiplier;
mod outputmix;
mod numops;

use stream::{Stream, OneSeqStream, NoSeqStream, SpecificSeqStream};
use multiplier::{Multiplier, DefaultMultiplier, McgMultiplier};
use outputmix::{OutputMixin, XshRsMixin, XshRrMixin};
use numops::*;

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

impl<Itype, Xtype, StreamMix, MulMix, OutMix> PcgEngine<Itype, Xtype, StreamMix, MulMix, OutMix> 
    where Itype: PcgConsts + PcgOps + BitSize + AsSmaller<Xtype> + Clone, Xtype: PcgOps + BitSize, 
        StreamMix: Stream<Itype>, MulMix: Multiplier<Itype>, OutMix: OutputMixin<Itype, Xtype> {
            
    pub fn new_unseeded() -> Self {
        PcgEngine {
            state      : Itype::zero(),
            stream_mix : StreamMix::build(),
            mul_mix    : MulMix::build(),
            out_mix    : OutMix::build(),
            phantom    : PhantomData::<Xtype>,
        }
    }        
}

//Provide random for 32 bit generators
impl<Itype, StreamMix, MulMix, OutMix> Rng for PcgEngine<Itype, u32, StreamMix, MulMix, OutMix>
    where Itype: PcgConsts + PcgOps + BitSize + AsSmaller<u32> + Clone,  
        StreamMix: Stream<Itype>, MulMix: Multiplier<Itype>, OutMix: OutputMixin<Itype, u32> {

    #[inline]
    fn next_u32(&mut self) -> u32 {
        let oldstate = self.state.clone();
        self.state = self.stream_mix.increment().add(oldstate.mul(self.mul_mix.multiplier()));
        
        self.out_mix.output(oldstate)
    }
}

//Provide random for 64 bit generators
impl<Itype, StreamMix, MulMix, OutMix> Rng for PcgEngine<Itype, u64, StreamMix, MulMix, OutMix>
    where Itype: PcgConsts + PcgOps + BitSize + AsSmaller<u64> + Clone,  
        StreamMix: Stream<Itype>, MulMix: Multiplier<Itype>, OutMix: OutputMixin<Itype, u64> {

    #[inline]
    fn next_u32(&mut self) -> u32 {
        let oldstate = self.state.clone();
        self.state = self.stream_mix.increment().add(oldstate.mul(self.mul_mix.multiplier()));
        
        //Truncate the output
        self.out_mix.output(oldstate) as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        let oldstate = self.state.clone();
        self.state = self.stream_mix.increment().add(oldstate.mul(self.mul_mix.multiplier()));
        
        self.out_mix.output(oldstate)
    }
}

impl<Itype, Xtype, StreamMix, MulMix, OutMix> Rand for PcgEngine<Itype, Xtype, StreamMix, MulMix, OutMix> 
    where Itype: PcgConsts + PcgOps + BitSize + AsSmaller<Xtype> + Clone + Rand, Xtype: PcgOps + BitSize, 
        StreamMix: Stream<Itype> + Rand, MulMix: Multiplier<Itype>, OutMix: OutputMixin<Itype, Xtype>
{
    fn rand<R: Rng>(rng: &mut R) -> Self {
        PcgEngine{
            state: rng.gen(),
            stream_mix : rng.gen(),
            mul_mix    : MulMix::build(),
            out_mix    : OutMix::build(),
            phantom    : PhantomData::<Xtype>,
        }
    }
}



pub type OneseqXshRs6432 = PcgEngine<u64, u32, OneSeqStream, DefaultMultiplier, XshRsMixin>;
pub type OneseqXshRr6432 = PcgEngine<u64, u32, OneSeqStream, DefaultMultiplier, XshRrMixin>;
pub type SetseqXshRs6432 = PcgEngine<u64, u32, SpecificSeqStream<u64>, DefaultMultiplier, XshRsMixin>;
pub type SetseqXshRr6432 = PcgEngine<u64, u32, SpecificSeqStream<u64>, DefaultMultiplier, XshRrMixin>;
pub type McgXshRs6432    = PcgEngine<u64, u32, NoSeqStream, McgMultiplier, XshRsMixin>;
pub type McgXshRr6432    = PcgEngine<u64, u32, NoSeqStream, McgMultiplier, XshRrMixin>;

/// A helper definition for a simple 32bit PCG which can have multiple random streams
pub type Pcg32       = SetseqXshRr6432;
/// A helper definition for a 32bit PCG which hase a fixed good random stream
pub type Pcg32Oneseq = OneseqXshRr6432;
/// A helper definition for a 32bit PCG which has a unique random stream for each instance
//pub type Pcg32Unique = UniqueXshRr6432;
/// A helper definition for a 32bit PCG which is fast but may lack statistical quality.
///
/// This generator sacrifices quality for speed by utilizing a Multiplicative Congruential
/// generator instead of a LCG. Additionally it uses a simpler permutation function so that the
/// compiler can optimize and reduce the number of operations.
pub type Pcg32Fast = McgXshRs6432;

pub type OneseqXshRs12864 = PcgEngine<u128, u64, OneSeqStream, DefaultMultiplier, XshRsMixin>;
pub type OneseqXshRr12864 = PcgEngine<u128, u64, OneSeqStream, DefaultMultiplier, XshRrMixin>;
pub type SetseqXshRs12864 = PcgEngine<u128, u64, SpecificSeqStream<u128>, DefaultMultiplier, XshRsMixin>;
pub type SetseqXshRr12864 = PcgEngine<u128, u64, SpecificSeqStream<u128>, DefaultMultiplier, XshRrMixin>;
pub type McgXshRs12864    = PcgEngine<u128, u64, NoSeqStream, McgMultiplier, XshRsMixin>;
pub type McgXshRr12864    = PcgEngine<u128, u64, NoSeqStream, McgMultiplier, XshRrMixin>;

/// A helper definition for a simple 64bit PCG which can have multiple random streams
pub type Pcg64       = SetseqXshRr12864;
/// A helper definition for a 64bit PCG which hase a fixed good random stream
pub type Pcg64Oneseq = OneseqXshRr12864;
/// A helper definition for a 64bit PCG which has a unique random stream for each instance
//pub type Pcg64Unique = UniqueXshRr12864;
/// A helper definition for a 64bit PCG which is fast but may lack statistical quality.
///
/// This generator sacrifices quality for speed by utilizing a Multiplicative Congruential
/// generator instead of a LCG. Additionally it uses a simpler permutation function so that the
/// compiler can optimize and reduce the number of operations.
pub type Pcg64Fast = McgXshRs12864;

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
