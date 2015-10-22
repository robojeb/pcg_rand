extern crate pcg_rand;
extern crate rand;

use rand::Rng;
use pcg_rand::Pcg32Basic;

fn main() {
    let mut rng = Pcg32Basic::new_unseeded();

    // print a bunch of random numbers
    println!("Here is a nice table of numbers:");
    for _ in 0..7 {
        println!("{: ^25}|{: ^25}|{: ^25}", rng.gen::<u32>(), rng.gen::<usize>(), rng.gen::<f64>());
    }

    //Later we will do party tricks
    //TODO: Implement the really big generators to get the party started
}
