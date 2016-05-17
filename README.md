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
This library currently provides 32 and 64 bit generators. 
It provides the major "stream" types, including the unique stream which is determined
by their current location in memory. 

Future Work
-----------

* ~~Implement more generator types~~
* Implement seeking for the generators
* Implement Extended generators (If I can figure out that code)
