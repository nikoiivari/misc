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
    let buffer: Vec<u8> = fs::read("symbols.exe").unwrap();
    let mut codesize:u32 = buffer[7] as u32; //lowest byte
    let cs2b: u32 = buffer[6] as u32;
    let cs3b: u32 = buffer[5] as u32;
    println!("{:?}", codesize);
    codesize = codesize & (cs2b << 8);
    println!("{:?}", codesize);
    codesize = codesize & (cs3b << 16); // 24bits of codesize
    println!("{:?}", codesize);
    // Put code into cache registers
    //TODO: codesize is broken. fix codesize.
}