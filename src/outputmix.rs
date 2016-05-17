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

pub trait OutputMixin<Itype, Xtype> {
    fn build() -> Self;
    fn output(&self, state : Itype) -> Xtype;
}

pub struct XshRsMixin;

impl<Itype, Xtype> OutputMixin<Itype, Xtype> for XshRsMixin 
    where Itype: PcgOps + AsSmaller<Xtype> + BitSize, Xtype: BitSize {
    
    fn build() -> XshRsMixin {
        XshRsMixin
    }
    
    #[inline(always)]
    fn output(&self, state : Itype) -> Xtype {
        let mut state = state;
        let sparebits = Itype::bits() - Xtype::bits();
        
        let opbits : usize = if sparebits - 5 >= 64 { 5 } else
                        if sparebits - 4 >= 32 { 4 } else
                        if sparebits - 3 >= 16 { 3 } else 
                        if sparebits - 2 >= 4  { 2 } else
                        if sparebits - 1 >= 1  { 1 } else { 0 };
        let mask = (1 << opbits) -1;
        let maxrandshift = mask;
        let topspare = opbits;
        let bottomspare = sparebits - topspare;
        let xshift = topspare + (Xtype::bits()+maxrandshift)/2;
        
        let rshift = if opbits != 0 {
            (state.rsh(Itype::bits() - opbits)).usize() & mask
        } else {
            0
        };
        
        state = state.xor(state.rsh(xshift));
        (state.rsh(bottomspare - maxrandshift + rshift)).shrink()
    }        
}

pub struct XshRrMixin;

impl<Itype, Xtype> OutputMixin<Itype, Xtype> for XshRrMixin 
    where Itype: PcgOps + AsSmaller<Xtype> + BitSize, Xtype: BitSize + PcgOps {
    
    fn build() -> XshRrMixin {
        XshRrMixin
    }
    
    #[inline(always)]
    fn output(&self, state : Itype) -> Xtype {
        let mut state = state;
        
        let sparebits = Itype::bits() - Xtype::bits();
        let xtypebits = Xtype::bits();
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
            (state.rsh(Itype::bits() - opbits)).usize() & mask
        } else { 0 };
        let amprot = (rot << amplifier) & mask;
        state = state.xor(state.rsh(xshift));
        let result : Xtype = state.rsh(bottomspare).shrink();
        result.rrot(amprot)
    }        
}
