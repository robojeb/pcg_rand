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

extern crate pcg_rand;
extern crate rand;

#[cfg(not(feature = "u128"))]
use pcg_rand::Pcg32;
#[cfg(feature = "u128")]
use pcg_rand::Pcg64;
use rand::{FromEntropy, Rng};

#[cfg(not(test))]
fn main() {
    #[cfg(feature = "u128")]
    let mut seeds: Pcg64 = Pcg64::from_entropy();
    #[cfg(not(feature = "u128"))]
    let mut seeds: Pcg32 = Pcg32::from_entropy();
    #[cfg(feature = "u128")]
    println!(
        "u128: 0x{:X} 0x{:X}",
        seeds.gen::<u128>(),
        seeds.gen::<u128>()
    );
    println!(
        "u64 : 0x{:X} 0x{:X}",
        seeds.gen::<u64>(),
        seeds.gen::<u64>()
    );
    println!(
        "u32 : 0x{:X} 0x{:X}",
        seeds.gen::<u32>(),
        seeds.gen::<u32>()
    );
    println!(
        "u16 : 0x{:X} 0x{:X}",
        seeds.gen::<u16>(),
        seeds.gen::<u16>()
    );
    println!("u8  : 0x{:X} 0x{:X}", seeds.gen::<u8>(), seeds.gen::<u8>());
}
