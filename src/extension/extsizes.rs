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
 */

/// This trait helps define the size of the extension of the PCG generator
/// this allows the size of the extension to effectively be part of the
/// type of the extended generator.
///
/// These extensions have to be a power of two based on the way the generator
/// utilizes its extension.
/// This could be made much better with associated constants and constant
/// functions, or by type level numbers.
///
/// Already provided are:
///  * Ext2
///  * Ext4
///  * Ext8
///  * Ext16
///  * Ext32
///  * Ext64
///  * Ext128
///  * Ext256
///  * Ext512
///  * Ext1024
/// Bigger extensions can be produced and dropped as long as they are powers of
/// 2
pub trait ExtSize {
    const EXT_SIZE: usize;
    const EXT_BITS: u32;
}

macro_rules! make_ext_size {
    ($($i:ident, $size:expr, $bits:expr);*) => {
        $(pub struct $i;

        impl ExtSize for $i {
            const EXT_SIZE: usize = $size;
            const EXT_BITS: u32 = $bits;
        })*
    }
}

make_ext_size!(
    Ext2, 2, 1;
    Ext4, 4, 2;
    Ext8, 8, 3;
    Ext16, 16, 4;
    Ext32, 32, 5;
    Ext64, 64, 6;
    Ext128, 128, 7;
    Ext256, 256, 8;
    Ext512, 512, 9;
    Ext1024, 1024, 10
);
