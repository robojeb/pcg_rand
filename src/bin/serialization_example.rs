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

extern crate pcg_rand;
extern crate rand;

#[cfg(feature = "serde1")]
extern crate serde_json;

#[cfg(feature = "serde1")]
use pcg_rand::{Pcg32, Pcg32Oneseq};

#[cfg(feature = "serde1")]
use rand::{SeedableRng, Rng};

#[cfg(all(not(test), feature = "serde1"))]
fn main() {
    let a: Pcg32 = Pcg32::from_entropy();
    let b: Pcg32Oneseq = Pcg32Oneseq::from_entropy();

    println!("{:?}", serde_json::to_string(&a));
    println!("{:?}", serde_json::to_string(&b));
}

#[cfg(not(feature = "serde1"))]
fn main() {
    println!("Test doesn't do much if you don't have serde enabled")
}
