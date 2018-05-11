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
//! pcg_rand = "0.7.0"
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
//! #How to Use
//! The simple generators work like the other Rng's from the `rand` crate.
//! You can create a PCG as follows
//!
//! ```
//! extern crate pcg_rand;
//! extern crate rand;
//!
//! use rand::Rng;
//! use pcg_rand::Pcg32;
//!
//! fn main() {
//!     let mut pcg = Pcg32::new_unseeded();
//! 
//!     let x : u32 = pcg.gen();
//! }
//! ```
//! 
//! The extended generators can be built in two ways, either by creating one 
//! directly, or by building them from a generator at its current state.
//!
//! ```
//! use pcg_rand::extension::{Pcg32Ext, ExtPcg, Ext256};
//! use pcg_rand::Pcg32Unique;
//! //Create an extended generator explicitly
//! let ext1 = Pcg32Ext::<Ext256>::new_unseeded();
//! 
//! //Create from another PCG
//! let ext2 : ExtPcg<_,_,_,_,_,Ext256> = ExtPcg::from_pcg(Pcg32Unique::new_unseeded());
//! ```

extern crate byteorder;
extern crate num_traits;
extern crate rand;
extern crate rand_core;

use rand_core::{RngCore, SeedableRng};

use std::num::Wrapping;

pub mod stream;
pub mod multiplier;
pub mod outputmix;
pub mod numops;
pub mod extension;
pub mod seeds;
    
use stream::{Stream, OneSeqStream, NoSeqStream, SpecificSeqStream, UniqueSeqStream};
use multiplier::{Multiplier, DefaultMultiplier, McgMultiplier};
use outputmix::{OutputMixin, XshRsMixin, XshRrMixin};
use numops::*;
use num_traits::Zero;

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
    mul_mix    : PhantomData<MulMix>,
    out_mix    : PhantomData<OutMix>,
    phantom    : PhantomData<Xtype>
}

impl<Itype, Xtype, StreamMix, MulMix, OutMix> PcgEngine<Itype, Xtype, StreamMix, MulMix, OutMix> 
    where 
    Itype: Zero,  
    StreamMix: Stream<Itype>, 
    MulMix: Multiplier<Itype>, 
    OutMix: OutputMixin<Itype, Xtype> {
            
    pub fn new_unseeded() -> Self {
        PcgEngine {
            state      : Itype::zero(),
            stream_mix : StreamMix::build(None),
            mul_mix    : PhantomData::<MulMix>,
            out_mix    : PhantomData::<OutMix>,
            phantom    : PhantomData::<Xtype>,
        }
    }        
}

//Provide random for 32 bit generators
impl<Itype, StreamMix, MulMix, OutMix> RngCore for PcgEngine<Itype, u32, StreamMix, MulMix, OutMix>
    where 
    Itype: PcgOps + Clone,  
    StreamMix: Stream<Itype>, 
    MulMix: Multiplier<Itype>, 
    OutMix: OutputMixin<Itype, u32> {

    fn next_u32(&mut self) -> u32 {
        let oldstate = self.state.clone();
        self.state = self.stream_mix.increment().wrap_add(oldstate.wrap_mul(MulMix::multiplier()));
        
        OutMix::output(oldstate)
    }

    fn next_u64(&mut self) -> u64 {
        rand_core::impls::next_u64_via_u32(self)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        rand_core::impls::fill_bytes_via_next(self, dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        Ok(self.fill_bytes(dest))
    }
}

//Provide random for 64 bit generators
impl<Itype, StreamMix, MulMix, OutMix> RngCore for PcgEngine<Itype, u64, StreamMix, MulMix, OutMix>
    where 
    Itype: PcgOps + Clone,
    StreamMix: Stream<Itype>, 
    MulMix: Multiplier<Itype>, 
    OutMix: OutputMixin<Itype, u64> {

    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    fn next_u64(&mut self) -> u64 {
        let oldstate = self.state.clone();
        self.state = self.stream_mix.increment().wrap_add(oldstate.wrap_mul(MulMix::multiplier()));
        
        OutMix::output(oldstate)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        rand_core::impls::fill_bytes_via_next(self, dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        Ok(self.fill_bytes(dest))
    }
}


pub type OneseqXshRs6432 = PcgEngine<u64, u32, OneSeqStream, DefaultMultiplier, XshRsMixin>;
pub type OneseqXshRr6432 = PcgEngine<u64, u32, OneSeqStream, DefaultMultiplier, XshRrMixin>;
pub type UniqueXshRs6432 = PcgEngine<u64, u32, UniqueSeqStream, DefaultMultiplier, XshRsMixin>;
pub type UniqueXshRr6432 = PcgEngine<u64, u32, UniqueSeqStream, DefaultMultiplier, XshRrMixin>;
pub type SetseqXshRs6432 = PcgEngine<u64, u32, SpecificSeqStream<u64>, DefaultMultiplier, XshRsMixin>;
pub type SetseqXshRr6432 = PcgEngine<u64, u32, SpecificSeqStream<u64>, DefaultMultiplier, XshRrMixin>;
pub type McgXshRs6432    = PcgEngine<u64, u32, NoSeqStream, McgMultiplier, XshRsMixin>;
pub type McgXshRr6432    = PcgEngine<u64, u32, NoSeqStream, McgMultiplier, XshRrMixin>;

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

pub type OneseqXshRs12832 = PcgEngine<u128, u32, OneSeqStream, DefaultMultiplier, XshRsMixin>;
pub type OneseqXshRr12832 = PcgEngine<u128, u32, OneSeqStream, DefaultMultiplier, XshRrMixin>;
pub type UniqueXshRs12832 = PcgEngine<u128, u32, UniqueSeqStream, DefaultMultiplier, XshRsMixin>;
pub type UniqueXshRr12832 = PcgEngine<u128, u32, UniqueSeqStream, DefaultMultiplier, XshRrMixin>;
pub type SetseqXshRs12832 = PcgEngine<u128, u32, SpecificSeqStream<u128>, DefaultMultiplier, XshRsMixin>;
pub type SetseqXshRr12832 = PcgEngine<u128, u32, SpecificSeqStream<u128>, DefaultMultiplier, XshRrMixin>;
pub type McgXshRs12832    = PcgEngine<u128, u32, NoSeqStream, McgMultiplier, XshRsMixin>;
pub type McgXshRr12832    = PcgEngine<u128, u32, NoSeqStream, McgMultiplier, XshRrMixin>;

/// A helper definition for a simple 32bit PCG which can have multiple random streams. This version uses 128bits of internal state
/// This makes it potentially slower but it has a longer period. (In testing
/// it appears to be better to use an extended generator Pcg32Ext to get a long
/// period rather than the Pcg32L)
pub type Pcg32L       = SetseqXshRr12832;
/// A helper definition for a 32bit PCG which hase a fixed good random streamThis version uses 128bits of internal state
/// This makes it potentially slower but it has a longer period.
pub type Pcg32LOneseq = OneseqXshRr12832;
/// A helper definition for a 32bit PCG which has a unique random stream for each instanceThis version uses 128bits of internal state
/// This makes it potentially slower but it has a longer period.
pub type Pcg32LUnique = UniqueXshRr12832;
/// A helper definition for a 32bit PCG which is fast but may lack statistical quality.
///
/// This generator sacrifices quality for speed by utilizing a Multiplicative Congruential
/// generator instead of a LCG. Additionally it uses a simpler permutation function so that the
/// compiler can optimize and reduce the number of operations.This version uses 128bits of internal state
/// This makes it potentially slower but it has a longer period.
pub type Pcg32LFast = McgXshRs12832;

pub type OneseqXshRs12864 = PcgEngine<u128, u64, OneSeqStream, DefaultMultiplier, XshRsMixin>;
pub type OneseqXshRr12864 = PcgEngine<u128, u64, OneSeqStream, DefaultMultiplier, XshRrMixin>;
pub type UniqueXshRs12864 = PcgEngine<u128, u64, UniqueSeqStream, DefaultMultiplier, XshRsMixin>;
pub type UniqueXshRr12864 = PcgEngine<u128, u64, UniqueSeqStream, DefaultMultiplier, XshRrMixin>;
pub type SetseqXshRs12864 = PcgEngine<u128, u64, SpecificSeqStream<u128>, DefaultMultiplier, XshRsMixin>;
pub type SetseqXshRr12864 = PcgEngine<u128, u64, SpecificSeqStream<u128>, DefaultMultiplier, XshRrMixin>;
pub type McgXshRs12864    = PcgEngine<u128, u64, NoSeqStream, McgMultiplier, XshRsMixin>;
pub type McgXshRr12864    = PcgEngine<u128, u64, NoSeqStream, McgMultiplier, XshRrMixin>;

/// A helper definition for a simple 64bit PCG which can have multiple random streams
pub type Pcg64       = SetseqXshRr12864;
/// A helper definition for a 64bit PCG which hase a fixed good random stream
pub type Pcg64Oneseq = OneseqXshRr12864;
/// A helper definition for a 64bit PCG which has a unique random stream for each instance
pub type Pcg64Unique = UniqueXshRr12864;
/// A helper definition for a 64bit PCG which is fast but may lack statistical quality.
///
/// This generator sacrifices quality for speed by utilizing a Multiplicative Congruential
/// generator instead of a LCG. Additionally it uses a simpler permutation function so that the
/// compiler can optimize and reduce the number of operations.
pub type Pcg64Fast = McgXshRs12864;


//
// Seeding for all of the different RNG types
//

impl<Itype, Xtype, StreamMix, MulMix, OutMix> SeedableRng for PcgEngine<Itype, Xtype, StreamMix, MulMix, OutMix> 
    where 
    Itype: Sized + seeds::ReadByteOrder,
    StreamMix: Stream<Itype>, 
    MulMix: Multiplier<Itype>, 
    OutMix: OutputMixin<Itype, Xtype>
{
    //FIXME: This is good enough for u128 (fine for now) but this type needs to 
    //be able to be dependant on the size of Itype
    type Seed = [u8; 32];

    fn from_seed(seed: Self::Seed) -> Self {
        let mut seeder = seeds::PCGSeeder::new(&seed);
        PcgEngine{
            state: seeder.get(),
            stream_mix : StreamMix::build(Some(seeder)),
            mul_mix    : PhantomData::<MulMix>,
            out_mix    : PhantomData::<OutMix>,
            phantom    : PhantomData::<Xtype>,
        }
    }
}

// impl<Itype, Xtype, MulMix, OutMix> SeedableRng for PcgEngine<Itype, Xtype, SpecificSeqStream<Itype>, MulMix, OutMix> 
//     where 
//     Itype: Clone,
//     MulMix: Multiplier<Itype>,
//     OutMix: OutputMixin<Itype, Xtype>,
//     SpecificSeqStream<Itype>: Stream<Itype>
// {
//     type Seed = [Itype;2];

//     fn from_seed(seed: Self::Seed) -> Self {
//         let mut stream = SpecificSeqStream::build();
//         stream.set_stream(seed[1].clone());
//         PcgEngine{
//             state: seed[0].clone(),
//             stream_mix : stream,
//             mul_mix    : PhantomData::<MulMix>,
//             out_mix    : PhantomData::<OutMix>,
//             phantom    : PhantomData::<Xtype>,
//         }
//     }
// }

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
impl RngCore for Pcg32Basic {
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

    fn next_u64(&mut self) -> u64 {
        rand_core::impls::next_u64_via_u32(self)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        rand_core::impls::fill_bytes_via_next(self, dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        Ok(self.fill_bytes(dest))
    }
}

//Allow seeding of Pcg32Basic
impl SeedableRng for Pcg32Basic {
    type Seed=[u8; std::mem::size_of::<u64>()*2];

    fn from_seed(seed: Self::Seed) -> Pcg32Basic {
        let mut seeder = seeds::PCGSeeder::new(&seed);        
        Pcg32Basic {
            state : seeder.get(),
            inc   : seeder.get(),
        }
    }
}
