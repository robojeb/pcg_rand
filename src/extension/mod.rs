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
 */

//! Extended PCG generators utilize a simple method to dramatically extend the 
//! period of the genrators. In addition to extending the period of the 
//! generator it ensures that the generators have K-dimensional
//! equidistribution. This means that the generator will produce every possible
//! K-tuple uniformly. 
//! 
//! These generators require `K*sizeof(Isize)` extra bytes to provide their 
//! equidistribution. 
//! 
//! These extended generators are currently in a beta state. They are 
//! implemented according to my understanding of the generator extension 
//! technique presented in the PCG paper. 
//! You can use these generators if you want, and if you would like to help
//! me review the code and determine if my implementation is correct that would
//! be wonderful. 

pub mod extsizes;

pub use self::extsizes::*;

use rand::{Rng, Rand, SeedableRng};
use std::marker::PhantomData;
use super::PcgEngine;
use super::numops::*;
use super::stream::*;
use super::outputmix::*;
use super::multiplier::*;
use num_traits::Zero;

macro_rules! extended_PCG {
    () => ()
}

/// An extended PCG generator. These generators provide K-dimensional 
/// equidistribution. Where K is specified by the value of the Size parameter
/// which must be an ExtSize type.
pub struct ExtPcg<Itype, Xtype, 
    StreamMix: Stream<Itype>, 
    MulMix: Multiplier<Itype>, 
    OutMix: OutputMixin<Itype, Xtype>, 
    Size: ExtSize> 
{
    pcg : PcgEngine<Itype, Xtype, StreamMix, MulMix, OutMix>,
    ext : Vec<Xtype>,
    _size : PhantomData<Size>,
}

impl<Itype, Xtype, StreamMix, MulMix, OutMix, Size> 
    ExtPcg<Itype, Xtype, StreamMix, MulMix, OutMix, Size> 
    where
    Itype: Zero,
    StreamMix: Stream<Itype>, 
    MulMix: Multiplier<Itype>, 
    OutMix: OutputMixin<Itype, Xtype>,
    Size: ExtSize,
    PcgEngine<Itype, Xtype, StreamMix, MulMix, OutMix>: Rng
{

    /// Create a new ExtPcg from an existing PCG. This will consume
    /// Size random values to initialize the extension array.
    pub fn from_pcg(pcg: PcgEngine<Itype, Xtype, StreamMix, MulMix, OutMix>) ->
        ExtPcg<Itype, Xtype, StreamMix, MulMix, OutMix, Size> {
            let mut pcg = pcg;

            //Create the starting extension array
            let mut ext = Vec::new();
            for _ in 0..Size::ext_size() {
                ext.push(pcg.gen());
            }

            ExtPcg {
                pcg : pcg,
                ext : ext,
                _size : PhantomData::<Size>,
            }
    }

    /// Create a new unseeded ExtPcg. 
    pub fn new_unseeded() -> ExtPcg<Itype, Xtype, StreamMix, MulMix, OutMix, Size> 
    {
        let pcg = PcgEngine::<Itype, Xtype, StreamMix, MulMix, OutMix>::new_unseeded();
        Self::from_pcg(pcg)
    }            
}

impl<Itype, StreamMix, MulMix, OutMix, Size> Rng for
    ExtPcg<Itype, u32, StreamMix, MulMix, OutMix, Size>
    where Itype: Clone, 
    StreamMix: Stream<Itype>, 
    MulMix: Multiplier<Itype>, 
    OutMix: OutputMixin<Itype, u32>,
    Size: ExtSize {

    #[inline]
    fn next_u32(&mut self) -> u32 {
        let oldstate = self.pcg.state.clone();
        self.pcg.state = self.pcg.stream_mix.increment().wrap_add(oldstate.wrap_mul(MulMix::multiplier()));

        let mask = 2usize.pow(Size::ext_bits() as u32)-1;
        let pick = self.pcg.state.as_usize() & mask;

        let ext_val = self.ext[pick];
        self.ext[pick] = self.ext[pick] + 1;
        OutMix::output(oldstate) ^ ext_val
    }
}

impl<Itype, StreamMix, MulMix, OutMix, Size> Rng for
    ExtPcg<Itype, u64, StreamMix, MulMix, OutMix, Size>
    where Itype: Clone, 
    StreamMix: Stream<Itype>, 
    MulMix: Multiplier<Itype>, 
    OutMix: OutputMixin<Itype, u64>,
    Size: ExtSize {

    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        let oldstate = self.pcg.state.clone();
        self.pcg.state = self.pcg.stream_mix.increment().wrap_add(oldstate.wrap_mul(MulMix::multiplier()));
        
        let mask = 2usize.pow(Size::ext_bits() as u32)-1;
        let pick = self.pcg.state.as_usize() & mask;

        let ext_val = self.ext[pick];
        self.ext[pick] = self.ext[pick] + 1;
        OutMix::output(oldstate) ^ ext_val
    }
}

pub type SetseqXshRr6432ext<Size> = ExtPcg<u64, u32, SpecificSeqStream<u64>, DefaultMultiplier, XshRrMixin, Size>;
pub type SetseqXshRr12832ext<Size> = ExtPcg<u128, u32, SpecificSeqStream<u128>, DefaultMultiplier, XshRrMixin, Size>;
pub type SetseqXshRr12864ext<Size> = ExtPcg<u128, u64, SpecificSeqStream<u128>, DefaultMultiplier, XshRrMixin, Size>;

/// The extended version of the Pcg32 generator
pub type Pcg32Ext<Size> = SetseqXshRr6432ext<Size>;
/// The extended version of the Pcg32L generator
pub type Pcg32LExt<Size> = SetseqXshRr12832ext<Size>;
/// The extended version of the Pcg64 generator
pub type Pcg64Ext<Size> = SetseqXshRr12864ext<Size>;

//
// Seeding for the ExtPcgs
//

// //These generics get pretty insane
// impl<Itype, Xtype, StreamMix, MulMix, OutMix, Size> SeedableRng<Itype> for ExtPcg<Itype, Xtype, StreamMix, MulMix, OutMix, Size> 
//     where 
//     Itype: Zero,
//     Xtype: PcgOps + BitSize + Rand, 
//     StreamMix: Stream<Itype>, 
//     MulMix: Multiplier<Itype>, 
//     OutMix: OutputMixin<Itype, Xtype>,
//     Size: ExtSize, ExtPcg<Itype, Xtype, StreamMix, MulMix, OutMix, Size> : Rng,
//     PcgEngine<Itype, Xtype, StreamMix, MulMix, OutMix> : Rng + SeedableRng<Itype>
    
// {
//     fn reseed(&mut self, seed: Itype) {
//         //Update the PCG
//         self.pcg.reseed(seed);

//         //Update the extension array
//         for i in 0..Size::ext_size() {
//             self.ext[i] = self.pcg.gen();
//         }
//     }
    
//     fn from_seed(seed: Itype) -> Self {
//         let pcg = PcgEngine::from_seed(seed);
//         ExtPcg::from_pcg(pcg)
//     }
// }

// impl<Itype, Xtype,MulMix, OutMix, Size> SeedableRng<[Itype; 2]> for ExtPcg<Itype, Xtype, SpecificSeqStream<Itype>, MulMix, OutMix, Size> 
//     where 
//     Itype: Zero,
//     Xtype: PcgOps + BitSize + Rand, 
//     SpecificSeqStream<Itype>: Stream<Itype>, 
//     MulMix: Multiplier<Itype>, 
//     OutMix: OutputMixin<Itype, Xtype>,
//     Size: ExtSize, ExtPcg<Itype, Xtype, SpecificSeqStream<Itype>, MulMix, OutMix, Size> : Rng,
//     PcgEngine<Itype, Xtype, SpecificSeqStream<Itype>, MulMix, OutMix> : Rng + SeedableRng<[Itype; 2]>
    
// {
//     fn reseed(&mut self, seed: [Itype; 2]) {
//         //Update the PCG
//         self.pcg.reseed(seed);

//         //Update the extension array
//         for i in 0..Size::ext_size() {
//             self.ext[i] = self.pcg.gen();
//         }
//     }
    
//     fn from_seed(seed: [Itype; 2]) -> Self {
//         let pcg = PcgEngine::from_seed(seed);
//         ExtPcg::from_pcg(pcg)
//     }
// }

// impl<Xtype, StreamMix, MulMix, OutMix, Size> SeedableRng<[u64; 2]> for ExtPcg<u128, Xtype, StreamMix, MulMix, OutMix, Size> 
//     where 
//     Xtype: PcgOps + BitSize + Rand, 
//     StreamMix: Stream<u128>, 
//     MulMix: Multiplier<u128>, 
//     OutMix: OutputMixin<u128, Xtype>,
//     Size: ExtSize, ExtPcg<u128, Xtype, StreamMix, MulMix, OutMix, Size> : Rng,
//     PcgEngine<u128, Xtype, StreamMix, MulMix, OutMix> : Rng + SeedableRng<[u64; 2]>
    
// {
//     fn reseed(&mut self, seed: [u64; 2]) {
//         //Update the PCG
//         self.pcg.reseed(seed);

//         //Update the extension array
//         for i in 0..Size::ext_size() {
//             self.ext[i] = self.pcg.gen();
//         }
//     }
    
//     fn from_seed(seed: [u64; 2]) -> Self {
//         let pcg = PcgEngine::from_seed(seed);
//         ExtPcg::from_pcg(pcg)
//     }
// }

// impl<Xtype,MulMix, OutMix, Size> SeedableRng<[u64; 4]> for ExtPcg<u128, Xtype, SpecificSeqStream<u128>, MulMix, OutMix, Size> 
//     where 
//     Xtype: PcgOps + BitSize + Rand, 
//     SpecificSeqStream<u128>: Stream<u128>, 
//     MulMix: Multiplier<u128>, 
//     OutMix: OutputMixin<u128, Xtype>,
//     Size: ExtSize, ExtPcg<u128, Xtype, SpecificSeqStream<u128>, MulMix, OutMix, Size> : Rng,
//     PcgEngine<u128, Xtype, SpecificSeqStream<u128>, MulMix, OutMix> : Rng + SeedableRng<[u64; 4]>
    
// {
//     fn reseed(&mut self, seed: [u64; 4]) {
//         //Update the PCG
//         self.pcg.reseed(seed);

//         //Update the extension array
//         for i in 0..Size::ext_size() {
//             self.ext[i] = self.pcg.gen();
//         }
//     }
    
//     fn from_seed(seed: [u64; 4]) -> Self {
//         let pcg = PcgEngine::from_seed(seed);
//         ExtPcg::from_pcg(pcg)
//     }
// }
