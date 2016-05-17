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

use ::numops::{PcgConsts, PcgOps};
use rand::{Rng, Rand};

pub trait Stream<Itype> {
    fn build() -> Self;
    
    fn set_stream(&mut self, _stream_seq : Itype){
        panic!("Stream setting unimplemented for this stream type");
    }

    fn increment(&self) -> Itype;

    fn get_stream(&self) -> Itype;
}

pub struct OneSeqStream;

impl<Itype: PcgConsts> Stream<Itype> for OneSeqStream {
    fn build() -> Self {
        OneSeqStream
    }
    
    fn increment(&self) -> Itype {
        Itype::stream()
    }
    
    fn get_stream(&self) -> Itype {
        Itype::stream()
    }
}

impl Rand for OneSeqStream {
    fn rand<R: Rng>(_rng: &mut R) -> Self {
        OneSeqStream
    }
}

pub struct NoSeqStream;

impl<Itype: PcgOps> Stream<Itype> for NoSeqStream {
    fn build() -> Self {
        NoSeqStream
    }
    
    fn increment(&self) -> Itype {
        Itype::zero()
    }
    
    fn get_stream(&self) -> Itype {
        Itype::zero()
    }
}

impl Rand for NoSeqStream {
    fn rand<R: Rng>(_rng: &mut R) -> Self {
        NoSeqStream
    }
}

pub struct SpecificSeqStream<Itype> {
    inc : Itype
}

impl<Itype: PcgOps + PcgConsts + Clone> Stream<Itype> for SpecificSeqStream<Itype> {
    fn build() -> Self {
        SpecificSeqStream {
            inc : Itype::stream(),
        }
    }
    
    fn set_stream(&mut self, stream_seq : Itype) {
        self.inc = stream_seq;
    }
    
    fn increment(&self) -> Itype {
        self.inc.or(Itype::one())
    }
    
    fn get_stream(&self) -> Itype {
        self.inc.or(Itype::one())
    }
}

impl<Itype: Rand> Rand for SpecificSeqStream<Itype> {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        SpecificSeqStream {
            inc : rng.gen(),
        }
    }
}


pub struct UniqueSeqStream;

impl<Itype: PcgOps> Stream<Itype> for UniqueSeqStream {
    fn build() -> Self {
        UniqueSeqStream
    }
    
    fn increment(&self) -> Itype {
        Itype::from_usize(self as *const UniqueSeqStream as usize | 1)
    }
    
    fn get_stream(&self) -> Itype {
        Itype::from_usize(self as *const UniqueSeqStream as usize | 1)
    }
}

impl Rand for UniqueSeqStream {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        UniqueSeqStream
    }
}
