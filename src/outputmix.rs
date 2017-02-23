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

use ::numops::*;
#[cfg(feature = "extprim_u128")]
use extprim::u128::u128 as eu128;

/// The output mixin trait provides the permutation function for the output
/// of the PCG. After the LCG state is advanced the state is run through
/// the `output(...)` function to produce the output. 
pub trait OutputMixin<Itype, Xtype> {
    fn output(&self, state : Itype) -> Xtype;
}

/// This output uses an Xor-shift followed by a right shift
pub struct XshRsMixin;

macro_rules! make_XshRsMixin {
    ($( $itype:ty=$ibits:expr, $xtype:ty=$xbits:expr );*) => (
        $(
            impl OutputMixin<$itype, $xtype> for XshRsMixin {
                
                #[inline(always)]
                fn output(&self, state : $itype) -> $xtype {
                    let mut state = state;
                    let sparebits = $ibits - $xbits;
                    
                    let opbits : usize = if sparebits - 5 >= 64 { 5 } else
                                    if sparebits - 4 >= 32 { 4 } else
                                    if sparebits - 3 >= 16 { 3 } else 
                                    if sparebits - 2 >= 4  { 2 } else
                                    if sparebits - 1 >= 1  { 1 } else { 0 };
                    let mask = (1 << opbits) -1;
                    let maxrandshift = mask;
                    let topspare = opbits;
                    let bottomspare = sparebits - topspare;
                    let xshift = topspare + ($xbits+maxrandshift)/2;
                    
                    let rshift = if opbits != 0 {
                        (state >> ($ibits - opbits)).as_usize() & mask
                    } else {
                        0
                    };
                    
                    state = state ^ (state >> xshift);
                    (state >> (bottomspare - maxrandshift + rshift)).shrink()
                }        
            }
        )*
    )
}

make_XshRsMixin! {
    u32=32, u8=8;
    u32=32, u16=16;
    u64=64, u16=16;
    u64=64, u32=32
}

#[cfg(feature = "extprim_u128")]
make_XshRsMixin! {
    eu128=128, u32=32;
    eu128=128, u64=64
}

#[cfg(feature = "i128_type")]
make_XshRsMixin! {
    u128=128, u32=32;
    u128=128, u64=64
}

/// This output uses an xor-shift followed by a random rotation.
pub struct XshRrMixin;

macro_rules! make_XshRrMixin {
    ($( $itype:ty=$ibits:expr, $xtype:ty=$xbits:expr );*) => ( 
        $(
            impl OutputMixin<$itype, $xtype> for XshRrMixin {
                
                #[inline(always)]
                fn output(&self, state : $itype) -> $xtype {
                    let mut state = state;
                    
                    let sparebits = $ibits - $xbits;
                    let xtypebits = $xbits;
                    let wantedopbits : usize = if xtypebits >= 128 { 7 } else 
                                        if xtypebits >= 64 { 6 } else 
                                        if xtypebits >= 32 { 5 } else
                                        if xtypebits >= 16 { 4 } else { 3 };
                    
                    let opbits : usize = if sparebits >= wantedopbits { 
                        wantedopbits 
                    } else { 
                        sparebits 
                    }; 
                    
                    let amplifier = wantedopbits - opbits;
                    let mask = (1 << opbits) - 1;
                    let topspare = opbits;
                    let bottomspare = sparebits - topspare;
                    let xshift = (topspare + xtypebits)/2;
                    
                    let rot = if opbits != 0 {
                        (state >> ($ibits - opbits)).as_usize() & mask
                    } else { 0 };

                    let amprot = (rot << amplifier) & mask;
                    state = state ^ (state >> xshift);

                    let result : $xtype = (state >> bottomspare).shrink();
                    result.rotate_right(amprot as u32)
                }        
            }
        )*
    )
}

make_XshRrMixin! {
    u32=32, u8=8;
    u32=32, u16=16;
    u64=64, u16=16;
    u64=64, u32=32
}

#[cfg(feature = "extprim_u128")]
make_XshRrMixin! {
    eu128=128, u32=32;
    eu128=128, u64=64
}

#[cfg(feature = "i128_type")]
make_XshRrMixin! {
    u128=128, u32=32;
    u128=128, u64=64
}
