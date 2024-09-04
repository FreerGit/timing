# Timing

[![build & test](https://github.com/freergit/timing/actions/workflows/ci.yml/badge.svg)](https://github.com/freergit/timing/actions/workflows/ci.yml)
[![license](https://img.shields.io/github/license/freergit/timing)](https://github.com/freergit/timing/blob/main/LICENSE.txt)


Easily measure executable time of a code block in human readable form. A rdtsc variant is also available for x86_64 architectures.


```rust
let time = timing(|| {
    println!("A fn that takes a few micros");
});

let (val, time) = timing_return(|| {
    println!("A fn that takes a few micros");
    5
});

let counter = timing_rdtsc(|| {
    println!("...");
});

let (val, counter) = timing_rdtsc_return(|| {
    println!("...");
    42
});

```