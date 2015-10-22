extern crate rand;

use rand::{Rng, Rand, SeedableRng};

use std::num::Wrapping;

pub struct Pcg32Basic {
    state : u64,
    inc   : u64,
}

impl Pcg32Basic {
    pub fn new_unseeded() -> Pcg32Basic {
        Pcg32Basic{
            state : 0,
            inc : 0,
        }
    }
}

//Pcg32Basic is an rng
impl Rng for Pcg32Basic {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        let oldstate = Wrapping(self.state);
        //Update the state as an lcg
        self.state = (oldstate * Wrapping(6364136223846793005u64) + Wrapping(self.inc | 1)).0;

        //Prepare the permutation on the output
        let xorshifted : u32 = (((oldstate >> 18usize) ^ oldstate) >> 27usize).0 as u32;
        let rot : u32 = (oldstate >> 59usize).0 as u32;

        //Produce the permuted output
        (xorshifted >> rot) | (xorshifted << ((-(rot as i32)) & 31))
    }
}

//Allow seeding of Pcg32Basic
impl SeedableRng<[u64; 2]> for Pcg32Basic {
    fn reseed(&mut self, seed: &mut [u64; 2]) {
        self.state = seed[0];
        self.inc   = seed[1];
    }

    fn from_seed(seed: [u64; 2]) -> Pcg32Basic {
        Pcg32Basic {
            state : seed[0],
            inc   : seed[1],
        }
    }
}

//Pcg32Basic can be randomly initialized with system entropy (or any other RNG)
impl Rand for Pcg32Basic {
    fn rand<R: Rng>(other: &mut R) -> Pcg32Basic {
        Pcg32Basic{
            state : other.gen(),
            inc   : other.gen(),
        }
    }
}


#[test]
fn it_works() {
    let mut rng = Pcg32Basic::new_unseeded();
}
