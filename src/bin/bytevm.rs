#![allow(dead_code, unused_imports)]

use std::env;
use std::fs;

struct Reg {
    wbr: bool,
    cap: bool,
    bits:u64,
}

// Bytecode virtual machine
// Read bytecode file and execute.

fn main () {
    // Cache of 4096 registers. (16 pages of 256 each). ~32KB cache
    let mut _cache: [Reg; 4096];

    // Accum of 4096 registers. (16 pages of 256 each). ~32KB accum
    let mut _accum: [Reg; 4096];

    
    // File loading and setup that an OS would do.
    // TODO: get filename from commandline!!!
    let _buffer: Vec<u8> = fs::read("symbols.exe").unwrap();

}