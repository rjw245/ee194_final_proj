parsec-3.0 is a stripped down version of parsec 3.0 core which contains only blackscholes. This will be used to profile blackscholes written in C.

The rust directory contains a blackscholes kernel (bs.c) adapted for linking with Rust. bs.rs is the blackscholes driver which links and runs the blackscholes kernel.
