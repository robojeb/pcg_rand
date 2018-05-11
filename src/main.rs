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

use rand::{RngCore, Rng, FromEntropy};
use pcg_rand::{Pcg32Basic, Pcg32, Pcg64, Pcg32Unique, Pcg32L};


#[cfg(not(test))]
fn main() {
    let mut rng = Pcg32Basic::new_unseeded();

    // print a bunch of random numbers
    println!("Here is the generator recovering from a (0,0) initialization: ");
    for _ in 0..10 {
        println!("{: ^25}|{: ^25}|{: ^25}", rng.gen::<u32>(), rng.gen::<u32>(), rng.gen::<u32>());
    }

    let mut rng : Pcg32 = Pcg32::from_entropy();
    println!("\nHere is the 32bit generator with random seed and increment: ");
    for _ in 0..10 {
        println!("{: ^25}|{: ^25}|{: ^25}", rng.gen::<u32>(), rng.gen::<u32>(), rng.gen::<u32>());
    }
    
    let mut rng : Pcg64 = Pcg64::from_entropy();
    println!("\nHere is the 64bit generator with random seed and increment: ");
    for _ in 0..10 {
        println!("{: ^25}|{: ^25}|{: ^25}", rng.gen::<u64>(), rng.gen::<u64>(), rng.gen::<u64>());
    }
    
    let mut rng : Pcg32L = Pcg32L::from_entropy();
    println!("\nHere is the 32bit generator with 128 bits of internal state random seed and increment: ");
    for _ in 0..10 {
        println!("{: ^25}|{: ^25}|{: ^25}", rng.gen::<u32>(), rng.gen::<u32>(), rng.gen::<u32>());
    }

    // //Later we will do party tricks
    // //TODO: Implement the really big generators to get the party started

    println!("\nHere we show off what two unique stream generators with the same seed can do");
    println!("Example Code:
    let mut urng1 = Pcg32Unique::new_unseeded();
    let mut urng2 = Pcg32Unique::new_unseeded();
    ");
    let mut urng1 = Pcg32Unique::new_unseeded();
    let mut urng2 = Pcg32Unique::new_unseeded();
    println!("{: ^25}|{: ^25}", "Generator 1", "Generator2");
    for _ in 0..25 {
        println!("{: ^25}|{: ^25}", urng1.gen::<u32>(), urng2.gen::<u32>());
    }
    println!("The RNGs use their location as a sequence so they diverge quickly");
}
