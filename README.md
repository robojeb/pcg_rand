pcg_rand
========

[![Clippy Linting Result](http://clippy.bashy.io/github/robojeb/pcg_rand/master/badge.svg)](http://clippy.bashy.io/github/robojeb/pcg_rand/master/log)
[![Crates.io Version](https://img.shields.io/crates/v/pcg_rand.svg)](https://crates.io/crates/pcg_rand)
![License](https://img.shields.io/crates/l/rustc-serialize.svg)


PCG stands for Permuted Congruential generators. They are a simple family of
random number generators which use the much denounced Linear Congruential
Generator as a base. To overcome the well known limitations of the LCG Generator
the PCG family utilizes permutation functions to permute the output. More
information about PCG can be found [here](http://pcg-random.org).

Current Status
--------------
Currently there is only one generator implemented. The Pcg32Basic generator is
based off of the "C minimal" implementation from [pcg-random.org](http://pcg-random.org).
It is not the most powerful PCG but it is very fast and simple.

Future Work
-----------

* Implement more generator types
* Implement seeking for the generators


Limiting Factors
----------------

None of the 64 bit PCG generators can be implemented without a u128
implementation. This is waiting on either extprim stabilizing or OverflowingOps
being stabilized. (Or for me to figure out how to write one myself, or use a
  C/C++ library)

This doesn't prevent the generators in this library from producing 64bit values
(because Rust's Rng provides an adaptor) but it does reduce the period of the
generators.
