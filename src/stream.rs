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

use ::numops::PcgOps;
use rand::{Rng, Rand};
use extprim::u128::u128;

pub trait Stream<Itype> {
    fn build() -> Self;
    
    fn set_stream(&mut self, _stream_seq : Itype){
        panic!("Stream setting unimplemented for this stream type");
    }

    fn increment(&self) -> Itype;

    fn get_stream(&self) -> Itype;
}

pub struct OneSeqStream;

macro_rules! make_one_seq {
    ( $( $t:ty => $e:expr);* ) => {
		$(impl Stream<$t> for OneSeqStream {
            fn build() -> Self {
                OneSeqStream
            }

            #[inline(always)]    
            fn increment(&self) -> $t {
                $e
            }
            
            fn get_stream(&self) -> $t {
                $e
            }
        })*
	}
}

make_one_seq!{
    u32  => 2891336453u32;
    u64  => 1442695040888963407u64;
    u128 => u128::from_parts(6364136223846793005,1442695040888963407)
}

impl Rand for OneSeqStream {
    fn rand<R: Rng>(_rng: &mut R) -> Self {
        OneSeqStream
    }
}

pub struct NoSeqStream;

macro_rules! make_no_seq {
    ( $( $t:ty => $e:expr);* ) => {
		$(impl Stream<$t> for NoSeqStream {
            fn build() -> Self {
                NoSeqStream
            }

            #[inline(always)]    
            fn increment(&self) -> $t {
                $e
            }
            
            fn get_stream(&self) -> $t {
                $e
            }
        })*
	}
}

make_no_seq!{
    u32  => 0;
    u64  => 0;
    u128 => u128::zero()
}

impl Rand for NoSeqStream {
    fn rand<R: Rng>(_rng: &mut R) -> Self {
        NoSeqStream
    }
}

pub struct SpecificSeqStream<Itype> {
    inc : Itype
}

macro_rules! make_set_seq {
    ( $( $t:ident => $e:expr);* ) => {
        $(impl Stream<$t> for SpecificSeqStream<$t> {
            fn build() -> Self {
                SpecificSeqStream {
                    inc : $e,
                }
            }

            fn set_stream(&mut self, stream_seq : $t) {
                self.inc = stream_seq.or( $t::one() );
            }

            #[inline(always)]    
            fn increment(&self) -> $t {
                self.inc
            }
            
            fn get_stream(&self) -> $t {
                self.inc
            }
        })*
    }
}

make_set_seq!{
    u32 => 2891336453u32;
    u64 => 1442695040888963407u64;
    u128 => u128::from_parts(6364136223846793005,1442695040888963407)
}

impl<Itype: Rand + PcgOps> Rand for SpecificSeqStream<Itype> {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        SpecificSeqStream {
            inc : rng.gen::<Itype>().or(Itype::one()),
        }
    }
}


pub struct UniqueSeqStream;

impl<Itype: PcgOps> Stream<Itype> for UniqueSeqStream {
    fn build() -> Self {
        UniqueSeqStream
    }
    
    #[inline(always)]
    fn increment(&self) -> Itype {
        Itype::from_usize(self as *const UniqueSeqStream as usize | 1)
    }
    
    fn get_stream(&self) -> Itype {
        Itype::from_usize(self as *const UniqueSeqStream as usize | 1)
    }
}

impl Rand for UniqueSeqStream {
    fn rand<R: Rng>(_rng: &mut R) -> Self {
        UniqueSeqStream
    }
}
