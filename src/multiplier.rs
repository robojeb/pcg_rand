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

/// This trait provides the multiplier for the internal LCG of the PCG generator
/// Implementing this trait for a struct will allow providing your own
/// multiplier for the PCG.
pub trait Multiplier<Itype> {
    fn multiplier() -> Itype;
}

/// Provides a default "good" multiplier based on the multiplier provided
/// in the C++ implementation of PCG
pub struct DefaultMultiplier;

macro_rules! make_default_mul {
	( $( $t:ty => $e:expr);* ) => {
		$(impl Multiplier<$t> for DefaultMultiplier {
			#[inline]
			fn multiplier() -> $t {
				$e
			}
		})*
	}
}

make_default_mul!(
    u8 => 141u8;
    u16 => 12829u16;
    u32 => 747_796_405u32;
    u64 => 6_364_136_223_846_793_005u64;
    u128 => 47_026_247_687_942_121_848_144_207_491_837_523_525u128 //u128::from_parts(2549297995355413924, 4865540595714422341)
);

/// Provides a default "good" multiplier based on the multiplier provided
/// in the C++ implementation of PCG for the MCG variant of the PCG generator.
pub struct McgMultiplier;

macro_rules! make_mcg_mul {
	     ( $( $t:ty => $e:expr);* ) => {
	       $( impl Multiplier<$t> for McgMultiplier {
	       	  #[inline]
		  fn multiplier() -> $t {
		     $e
		  }
		})*
	}
}

make_mcg_mul!(
    u8 => 217u8;
    u16 => 62169u16;
    u32 => 277_803_737u32;
    u64 => 12_605_985_483_714_917_081u64;
    u128 => 327_738_287_884_841_127_335_028_083_622_016_905_945u128//u128::from_parts(17766728186571221404,12605985483714917081)
);
