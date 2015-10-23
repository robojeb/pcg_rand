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

extern crate rand;

use rand::{Rng, Rand, SeedableRng};

use std::num::Wrapping;

mod stream;
mod multiplier;
mod outputmix;

use stream::{Stream, OneSeqStream, SpecificSeqStream, UniqueSeqStream};
use multiplier::{Multiplier, DefaultMultiplier};
use outputmix::{OutputMixin, XshRsMixin, XshRrMixin};

use std::marker::PhantomData;


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
            pub type $name = PcgEngine<$itype, $xtype, $seq, $mul, $out>;

            impl $name {
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
        )*
    )
}

macro_rules! build_sequence_pcg {
    ( $($name:ident, $itype:ty, $xtype:ty, $seq:ident, $mul:ident, $out:ident);* ) => (
        $(
            pub type $name = PcgEngine<$itype, $xtype, $seq<$itype>, $mul, $out>;

            impl $name {
                pub fn new_unseeded() -> $name {
                    PcgEngine{
                        state      : 0,
                        stream_mix : $seq::<$itype>::new(),
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
        )*
    )
}

build_basic_pcg!(
    oneseq_xsh_rs_64_32, u64, u32, OneSeqStream, DefaultMultiplier, XshRsMixin;
    unique_xsh_rs_64_32, u64, u32, UniqueSeqStream, DefaultMultiplier, XshRsMixin;
    oneseq_xsh_rr_64_32, u64, u32, OneSeqStream, DefaultMultiplier, XshRrMixin;
    unique_xsh_rr_64_32, u64, u32, UniqueSeqStream, DefaultMultiplier, XshRrMixin
);

build_sequence_pcg!(
    setseq_xsh_rs_64_32, u64, u32, SpecificSeqStream, DefaultMultiplier, XshRsMixin;
    setseq_xsh_rr_64_32, u64, u32, SpecificSeqStream, DefaultMultiplier, XshRrMixin
);

pub type Pcg32        = setseq_xsh_rr_64_32;
pub type Pcg32_oneseq = oneseq_xsh_rr_64_32;
pub type Pcg32_unique = unique_xsh_rr_64_32;

/*
 * The simple C minimal implementation of PCG32
 */

pub struct Pcg32_basic {
    state : u64,
    inc   : u64,
}

impl Pcg32_basic {
    pub fn new_unseeded() -> Pcg32_basic {
        Pcg32_basic{
            state : 0,
            inc : 0,
        }
    }
}

//Pcg32_basic is an rng
impl Rng for Pcg32_basic {
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

//Allow seeding of Pcg32_basic
impl SeedableRng<[u64; 2]> for Pcg32_basic {
    fn reseed(&mut self, seed: [u64; 2]) {
        self.state = seed[0];
        self.inc   = seed[1];
    }

    fn from_seed(seed: [u64; 2]) -> Pcg32_basic {
        Pcg32_basic {
            state : seed[0],
            inc   : seed[1],
        }
    }
}

//Pcg32_basic can be randomly initialized with system entropy (or any other RNG)
impl Rand for Pcg32_basic {
    fn rand<R: Rng>(other: &mut R) -> Pcg32_basic {
        Pcg32_basic{
            state : other.gen(),
            inc   : other.gen(),
        }
    }
}
