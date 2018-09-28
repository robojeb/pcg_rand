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

use super::multiplier::*;
use super::numops::*;
use super::outputmix::*;
use super::seeds::PcgSeeder;
use super::stream::*;
use super::PcgEngine;
use num_traits::{One, Zero};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use rand_core::{RngCore, SeedableRng};
use std::marker::PhantomData;

/// An extended PCG generator. These generators provide K-dimensional
/// equidistribution. Where K is specified by the value of the Size parameter
/// which must be an ExtSize type.
pub struct ExtPcg<
    Itype,
    Xtype,
    StreamMix: Stream<Itype>,
    MulMix: Multiplier<Itype>,
    OutMix: OutputMixin<Itype, Xtype>,
    Size: ExtSize,
> {
    pcg: PcgEngine<Itype, Xtype, StreamMix, MulMix, OutMix>,
    ext: Vec<Xtype>,
    _size: PhantomData<Size>,
}

impl<Itype, Xtype, StreamMix, MulMix, OutMix, Size>
    ExtPcg<Itype, Xtype, StreamMix, MulMix, OutMix, Size>
where
    Itype: Zero + One,
    Xtype: PcgOps + BitSize,
    Standard: Distribution<Xtype>,
    StreamMix: Stream<Itype>,
    MulMix: Multiplier<Itype>,
    OutMix: OutputMixin<Itype, Xtype>,
    Size: ExtSize,
    PcgEngine<Itype, Xtype, StreamMix, MulMix, OutMix>: Rng + SeedableRng,
{
    /// Create a new ExtPcg from an existing PCG. This will consume
    /// Size random values to initialize the extension array.
    pub fn from_pcg(
        pcg: PcgEngine<Itype, Xtype, StreamMix, MulMix, OutMix>,
    ) -> ExtPcg<Itype, Xtype, StreamMix, MulMix, OutMix, Size> {
        let mut pcg = pcg;

        //Create the starting extension array
        let mut ext = Vec::with_capacity(Size::EXT_SIZE);
        for _ in 0..Size::EXT_SIZE {
            ext.push(pcg.gen());
        }

        ExtPcg {
            pcg,
            ext,
            _size: PhantomData::<Size>,
        }
    }
}

impl<Itype, Xtype, StreamMix, MulMix, OutMix, Size>
    ExtPcg<Itype, Xtype, StreamMix, MulMix, OutMix, Size>
where
    Itype: Zero + One,
    Xtype: PcgOps + BitSize,
    Standard: Distribution<Xtype>,
    StreamMix: Stream<Itype>,
    MulMix: Multiplier<Itype>,
    OutMix: OutputMixin<Itype, Xtype>,
    Size: ExtSize,
    PcgEngine<Itype, Xtype, StreamMix, MulMix, OutMix>: Rng + SeedableRng,
    PcgSeeder<Itype>: Default,
{
    /// Creates a new ePCG without specifying a seed.
    /// WARNING: Every PCG created with this method will produce the same
    /// output. In most cases a seeded PCG will be more useful, please check
    /// the references for `rand::SeedableRng` and `rand::FromEntropy` for
    /// methods to seed a ePCG.
    pub fn new_unseeded() -> ExtPcg<Itype, Xtype, StreamMix, MulMix, OutMix, Size> {
        let pcg = PcgEngine::<Itype, Xtype, StreamMix, MulMix, OutMix>::new_unseeded();
        Self::from_pcg(pcg)
    }
}

impl<Itype, StreamMix, MulMix, OutMix, Size> RngCore
    for ExtPcg<Itype, u32, StreamMix, MulMix, OutMix, Size>
where
    Itype: PcgOps + AsUsize + BitSize + AsSmaller<u32> + Clone,
    StreamMix: Stream<Itype>,
    MulMix: Multiplier<Itype>,
    OutMix: OutputMixin<Itype, u32>,
    Size: ExtSize,
{
    #[inline]
    fn next_u32(&mut self) -> u32 {
        let oldstate = self.pcg.state.clone();
        self.pcg.state = self
            .pcg
            .stream_mix
            .increment()
            .wrap_add(oldstate.wrap_mul(MulMix::multiplier()));

        let mask = 2usize.pow(Size::EXT_BITS) - 1;
        let pick = self.pcg.state.as_usize() & mask;

        let ext_val = self.ext[pick];
        self.ext[pick] += 1;
        OutMix::output(oldstate) ^ ext_val
    }

    fn next_u64(&mut self) -> u64 {
        ::rand_core::impls::next_u64_via_u32(self)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        ::rand_core::impls::fill_bytes_via_next(self, dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), ::rand_core::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

impl<Itype, StreamMix, MulMix, OutMix, Size> RngCore
    for ExtPcg<Itype, u64, StreamMix, MulMix, OutMix, Size>
where
    Itype: PcgOps + AsUsize + BitSize + AsSmaller<u64> + Clone,
    StreamMix: Stream<Itype>,
    MulMix: Multiplier<Itype>,
    OutMix: OutputMixin<Itype, u64>,
    Size: ExtSize,
{
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    fn next_u64(&mut self) -> u64 {
        let oldstate = self.pcg.state.clone();
        self.pcg.state = self
            .pcg
            .stream_mix
            .increment()
            .wrap_add(oldstate.wrap_mul(MulMix::multiplier()));

        let mask = 2usize.pow(Size::EXT_BITS) - 1;
        let pick = self.pcg.state.as_usize() & mask;

        let ext_val = self.ext[pick];
        self.ext[pick] += 1;
        OutMix::output(oldstate) ^ ext_val
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        ::rand_core::impls::fill_bytes_via_next(self, dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), ::rand_core::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

pub type SetseqXshRr6432ext<Size> =
    ExtPcg<u64, u32, SpecificSeqStream<u64>, DefaultMultiplier, XshRrMixin, Size>;
pub type SetseqXshRr12832ext<Size> =
    ExtPcg<u128, u32, SpecificSeqStream<u128>, DefaultMultiplier, XshRrMixin, Size>;
pub type SetseqXshRr12864ext<Size> =
    ExtPcg<u128, u64, SpecificSeqStream<u128>, DefaultMultiplier, XshRrMixin, Size>;

/// The extended version of the Pcg32 generator
pub type Pcg32Ext<Size> = SetseqXshRr6432ext<Size>;
/// The extended version of the Pcg32L generator
pub type Pcg32LExt<Size> = SetseqXshRr12832ext<Size>;
/// The extended version of the Pcg64 generator
pub type Pcg64Ext<Size> = SetseqXshRr12864ext<Size>;

//
// Seeding for the ExtPcgs
//

//These generics get pretty insane
impl<Itype, Xtype, StreamMix, MulMix, OutMix, Size> SeedableRng
    for ExtPcg<Itype, Xtype, StreamMix, MulMix, OutMix, Size>
where
    Itype: ::seeds::ReadByteOrder + Default + Zero + One,
    Xtype: PcgOps + BitSize,
    Standard: Distribution<Xtype>,
    StreamMix: Stream<Itype>,
    MulMix: Multiplier<Itype>,
    OutMix: OutputMixin<Itype, Xtype>,
    Size: ExtSize,
    ExtPcg<Itype, Xtype, StreamMix, MulMix, OutMix, Size>: RngCore,
    PcgEngine<Itype, Xtype, StreamMix, MulMix, OutMix>:
        RngCore + SeedableRng<Seed = PcgSeeder<Itype>>,
    PcgSeeder<Itype>: Default,
{
    type Seed = PcgSeeder<Itype>;

    fn from_seed(seed: Self::Seed) -> Self {
        let pcg = PcgEngine::from_seed(seed);
        ExtPcg::from_pcg(pcg)
    }
}
