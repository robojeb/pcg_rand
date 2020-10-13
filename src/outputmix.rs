/*
 * PCG Random Number Generation for Rust
 *
 * Copyright 2015 John Brooks <jeb@robojeb.dev>
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

use num_traits::{One, PrimInt};
use numops::*;
use std::ops::{BitOr, BitXor, Shr};

/// The output mixin trait provides the permutation function for the output
/// of the PCG. After the LCG state is advanced the state is run through
/// the `output(...)` function to produce the output.
pub trait OutputMixin<Itype, Xtype> {
    const SERIALIZER_ID: &'static str;
    fn output(state: Itype, increment: Itype, multiplier: Itype) -> Xtype;
}

/// This output uses an Xor-shift followed by a right shift
pub struct XshRsMixin;

impl<Itype, Xtype> OutputMixin<Itype, Xtype> for XshRsMixin
where
    Itype: Shr<usize, Output = Itype>
        + BitXor<Itype, Output = Itype>
        + AsSmaller<Xtype>
        + BitSize
        + AsUsize
        + Copy,
    Xtype: BitSize,
{
    const SERIALIZER_ID: &'static str = "XshRs";
    #[inline(always)]
    fn output(state: Itype, _increment: Itype, _multiplier: Itype) -> Xtype {
        let mut state = state;
        let sparebits = Itype::BITS - Xtype::BITS;

        let opbits: usize = if sparebits - 5 >= 64 {
            5
        } else if sparebits - 4 >= 32 {
            4
        } else if sparebits - 3 >= 16 {
            3
        } else if sparebits - 2 >= 4 {
            2
        } else if sparebits > 1 {
            1
        } else {
            0
        };
        let mask = (1 << opbits) - 1;
        let maxrandshift = mask;
        let topspare = opbits;
        let bottomspare = sparebits - topspare;
        let xshift = topspare + (Xtype::BITS + maxrandshift) / 2;

        let rshift = if opbits != 0 {
            (state >> (Itype::BITS - opbits)).as_usize() & mask
        } else {
            0
        };

        state = state ^ (state >> xshift);
        (state >> (bottomspare - maxrandshift + rshift)).shrink()
    }
}

/// This output uses an xor-shift followed by a random rotation.
pub struct XshRrMixin;

impl<Itype, Xtype> OutputMixin<Itype, Xtype> for XshRrMixin
where
    Itype: Shr<usize, Output = Itype>
        + BitXor<Itype, Output = Itype>
        + AsUsize
        + AsSmaller<Xtype>
        + BitSize
        + Copy,
    Xtype: BitSize + PrimInt,
{
    const SERIALIZER_ID: &'static str = "XshRr";
    #[inline(always)]
    fn output(state: Itype, _increment: Itype, _multiplier: Itype) -> Xtype {
        let mut state = state;

        let sparebits = Itype::BITS - Xtype::BITS;
        let xtypebits = Xtype::BITS;
        let wantedopbits: usize = if xtypebits >= 128 {
            7
        } else if xtypebits >= 64 {
            6
        } else if xtypebits >= 32 {
            5
        } else if xtypebits >= 16 {
            4
        } else {
            3
        };

        let opbits: usize = if sparebits >= wantedopbits {
            wantedopbits
        } else {
            sparebits
        };

        let amplifier = wantedopbits - opbits;
        let mask = (1 << opbits) - 1;
        let topspare = opbits;
        let bottomspare = sparebits - topspare;
        let xshift = (topspare + xtypebits) / 2;

        let rot = if opbits != 0 {
            (state >> (Itype::BITS - opbits)).as_usize() & mask
        } else {
            0
        };

        let amprot = (rot << amplifier) & mask;
        state = state ^ (state >> xshift);

        let result: Xtype = (state >> bottomspare).shrink();
        result.rotate_right(amprot as u32)
    }
}

/// The Double Xor-shift multiply output
/// This is a new (added to the PCG C++ library in 2019) output which is meant to be more powerful.
pub struct DXsMMixin;

impl<Itype, Xtype> OutputMixin<Itype, Xtype> for DXsMMixin
where
    Itype: AsSmaller<Xtype> + Shr<usize, Output = Itype> + BitSize + Copy,
    Xtype: BitSize
        + PcgOps
        + Shr<usize, Output = Xtype>
        + BitXor<Xtype, Output = Xtype>
        + BitOr<Xtype, Output = Xtype>
        + One
        + Copy,
{
    const SERIALIZER_ID: &'static str = "DXsM";
    #[inline(always)]
    fn output(state: Itype, _increment: Itype, multiplier: Itype) -> Xtype {
        let hi: Xtype = (state >> (Itype::BITS - Xtype::BITS)).shrink();
        let low: Xtype = state.shrink();

        let low = low | Xtype::one();
        let hi = hi ^ (hi >> (Xtype::BITS / 2));

        let hi = hi.wrap_mul(multiplier.shrink());

        let hi = hi ^ (hi >> (3 * Xtype::BITS / 4));

        hi.wrap_mul(low)
    }
}
