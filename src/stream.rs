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
#[cfg(feature = "extprim_u128")]
use extprim::u128::u128 as eu128;
use num_traits::{One, FromPrimitive};
use std::ops::BitOr;

use std::marker::PhantomData;

/// A stream provides the increment to the LCG. This increment should be
/// an odd number or the period of the generator will not be the full size
/// of the state.
pub trait Stream<Itype> {
    fn build() -> Self;
    
    fn set_stream(&mut self, _stream_seq : Itype){
        panic!("Stream setting unimplemented for this stream type");
    }

    fn increment(&self) -> Itype;

    fn get_stream(&self) -> Itype;
}

/// This sequence stream defines constants as provided by the PCG paper.
/// This struct is implemented with a macro to provide values for each
/// Stream<Itype>.
pub struct OneSeqStream<Itype> {
    phantom: PhantomData<Itype>,   
}

macro_rules! make_one_seq {
    ( $( $t:ty => $e:expr);* ) => {
		$(impl Stream<$t> for OneSeqStream<$t> {
            fn build() -> Self {
                OneSeqStream{
                    phantom: PhantomData::<$t>,
                }
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
    u64  => 1442695040888963407u64
}

#[cfg(feature = "extprim_u128")]
make_one_seq!{
    eu128 => eu128::from_parts(6364136223846793005,1442695040888963407)
}

impl<Itype> Rand for OneSeqStream<Itype> 
    where OneSeqStream<Itype> : Stream<Itype> 
{
    fn rand<R: Rng>(_rng: &mut R) -> Self {
        OneSeqStream::build()
    }
}

/// This stream provides an increment of 0 to the LCG. This turns the
/// LCG into a MCG, which while being less statistically sound than an LCG,
/// it is faster.
pub struct NoSeqStream<Itype> {
    phantom: PhantomData<Itype>,   
}

macro_rules! make_no_seq {
    ( $( $t:ty => $e:expr);* ) => {
		$(impl Stream<$t> for NoSeqStream<$t> {
            fn build() -> Self {
                NoSeqStream{
                    phantom: PhantomData::<$t>,
                }
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
    u64  => 0
}

#[cfg(feature = "extprim_u128")]
make_no_seq!{
    eu128 => eu128::zero()
}

impl<Itype> Rand for NoSeqStream<Itype>  
    where NoSeqStream<Itype> : Stream<Itype> 
{
    fn rand<R: Rng>(_rng: &mut R) -> Self {
        NoSeqStream::build()
    }
}

/// By default this stream provides the same stream as OneSeqStream. The
/// advantage to this stream is it can be changed at runtime. This incurs an
/// extra Itype of storage overhead. 
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
                self.inc = stream_seq | $t::one();
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
    u64 => 1442695040888963407u64
    
}

#[cfg(feature = "extprim_u128")]
make_set_seq!{
    eu128 => eu128::from_parts(6364136223846793005,1442695040888963407)
}

impl<Itype: Rand> Rand for SpecificSeqStream<Itype> 
    where 
    Itype: Rand + BitOr<Itype, Output=Itype> + One
{
    fn rand<R: Rng>(rng: &mut R) -> Self {
        SpecificSeqStream {
            inc : rng.gen::<Itype>() | Itype::one(),
        }
    }
}

/// This stream provides a stream based on the current location of the 
/// generator in memory. This means that two PCG with the same seed 
/// can produce different sequences of numbers. Though if the generator is
/// moved it will change the stream.
pub struct UniqueSeqStream<Itype> {
    phantom: PhantomData<Itype>,   
}

impl<Itype> Stream<Itype> for UniqueSeqStream<Itype>
    where 
    Itype: FromPrimitive {
    fn build() -> Self {
        UniqueSeqStream {
            phantom: PhantomData::<Itype>,
        }
    }
    
    #[inline(always)]
    fn increment(&self) -> Itype {
        Itype::from_usize(self as *const UniqueSeqStream<Itype> as usize | 1).unwrap()
    }
    
    fn get_stream(&self) -> Itype {
        Itype::from_usize(self as *const UniqueSeqStream<Itype> as usize | 1).unwrap()
    }
}

impl<Itype> Rand for UniqueSeqStream<Itype>  
    where UniqueSeqStream<Itype> : Stream<Itype> 
{
    fn rand<R: Rng>(_rng: &mut R) -> Self {
        UniqueSeqStream::build()
    }
}
