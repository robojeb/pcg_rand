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

pub trait OutputMixin<Itype, Xtype> {
    fn output(&self, state : Itype) -> Xtype;
}

pub struct XshRsMixin;

macro_rules! make_Xsh_Rs_mixins {
    ( $( $it:ty, $xt:ty, $ibits:expr, $xbits:expr);*) => {
        $(impl OutputMixin<$it, $xt> for XshRsMixin {
            fn output(&self, state : $it) -> $xt {
                let mut state = state;

                const BITS : usize= $ibits;
                const XTYPEBITS : usize= $xbits;
                const SPAREBITS : usize = BITS - XTYPEBITS;
                //Have to use "let" right now because if complains when used in a const context
                let opbits : usize = if SPAREBITS-5 >= 64 { 5 } else
                             if SPAREBITS-4 >= 32 { 4 } else
                             if SPAREBITS-3 >= 16 { 3 } else
                             if SPAREBITS-2 >= 4  { 2 } else
                             if SPAREBITS-1 >= 1  { 1 } else { 0 };
                let mask = (1 << opbits) - 1;
                let maxrandshift = mask;
                let topspare = opbits;
                let bottomspare = SPAREBITS - topspare;
                let xshift = topspare + (XTYPEBITS+maxrandshift)/2;
                //Now we start real computation. Everything above was constexpr in the
                //C++ code originally
                let rshift = if opbits != 0 { (state >> (BITS - opbits)) as usize & mask }
                    else { 0 };

                state ^= state >> xshift;
                (state >> (bottomspare - maxrandshift + rshift)) as $xt
            }
        }
        )*
    }
}

make_Xsh_Rs_mixins!(
    u16, u8, 16, 8;
    u32, u16, 32, 16;
    u64, u32, 64, 32
);

pub struct XshRrMixin;

macro_rules! make_Xsh_Rr_mixins {
    ( $( $it:ty, $xt:ty, $ibits:expr, $xbits:expr);*) => {
        $(impl OutputMixin<$it, $xt> for XshRrMixin {
            fn output(&self, state : $it) -> $xt {
                let mut state = state;

                const BITS : usize= $ibits;
                const XTYPEBITS : usize= $xbits;
                const SPAREBITS : usize = BITS - XTYPEBITS;
                //Have to use "let" right now because if complains when used in a const context
                let wantedopbits : usize = if XTYPEBITS >= 128 { 7 } else
                             if XTYPEBITS >= 64 { 6 } else
                             if XTYPEBITS >= 32 { 5 } else
                             if XTYPEBITS >= 16 { 4 } else { 3 };

                let opbits = if SPAREBITS >= wantedopbits { wantedopbits } else { SPAREBITS };
                let amplifier = wantedopbits - opbits;
                let mask = (1 << opbits) - 1;
                let topspare = opbits;
                let bottomspare = SPAREBITS - topspare;
                let xshift = (topspare + XTYPEBITS)/2;

                //Begin math from the C++
                let rot = if opbits != 0 { (state >> (BITS - opbits)) as usize & mask } else {0};
                let amprot = (rot << amplifier) & mask;
                state ^= state >> xshift;
                let result : $xt = (state >> bottomspare) as $xt;
                result.rotate_right(amprot as u32)
            }
        }
        )*
    }
}

make_Xsh_Rr_mixins!(
    u16, u8, 16, 8;
    u32, u16, 32, 16;
    u64, u32, 64, 32
);
