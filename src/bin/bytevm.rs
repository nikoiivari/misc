#![allow(dead_code, unused_imports)]

use std::env;
use std::fs;

#[derive(Clone)]
#[derive(Copy)]
struct Reg {
    wbr: bool,
    cap: bool,
    bits:u64,
}

// Handy when testing:
// echo -en "\xde\xad\xbe\xef\xde\xad\xbe\xef" >> symbols.xe
// repeat three times to get three registers of "code".

// To dump 8 bytes per line as hexadecimal:
// od -A x -t x1z -w8 symbols.xe

// Bytecode virtual machine
// Read bytecode file and execute.

fn main () {
    // Cache of 4096 registers. (16 pages of 256 each). ~32KB cache
    let mut cache: [Reg; 4096] = [Reg {wbr: false, cap: false, bits: 0}; 4096];

    // Accum of 4096 registers. (16 pages of 256 each). ~32KB accum
    let mut _accum: [Reg; 4096];

    
    // File loading and setup that an OS would do.
    let offset = 256; // 256 registers offset, to skip zero page
    // TODO: get filename from commandline!!!
    let buffer: Vec<u8> = fs::read("symbols.exe").unwrap();
    let mut codesize:u32 = buffer[7] as u32; //lowest byte
    let cs2b: u32 = buffer[6] as u32;
    let cs3b: u32 = buffer[5] as u32;
    //println!("{:x}", codesize);
    //println!("{:x}", cs2b);
    //println!("{:x}", cs3b);
    codesize = codesize | (cs2b << 8);
    //println!("{:x}", codesize);
    codesize = codesize | (cs3b << 16); // 24bits of codesize
    println!("{:x}", codesize);
    
    // Put code into cache registers. Start from offset.
    for i in offset..(offset + codesize) {
        cache[i as usize].wbr = false;
        cache[i as usize].cap = false;
        
        // each of the 8 bytes
        let j:usize = (i - offset) as usize;
        let dot_s = buffer[8 + (j * 8) + 0] as u64;
        let dot_t = buffer[8 + (j * 8) + 1] as u64;
        let dot_u = buffer[8 + (j * 8) + 2] as u64;
        let dot_v = buffer[8 + (j * 8) + 3] as u64;
        let dot_w = buffer[8 + (j * 8) + 4] as u64;
        let dot_x = buffer[8 + (j * 8) + 5] as u64;
        let dot_y = buffer[8 + (j * 8) + 6] as u64;
        let dot_z = buffer[8 + (j * 8) + 7] as u64;

        let mut bytes: u64 = dot_s << 56;
        bytes = bytes | (dot_t << 48);
        bytes = bytes | (dot_u << 40);
        bytes = bytes | (dot_v << 32);
        bytes = bytes | (dot_w << 24);
        bytes = bytes | (dot_x << 16);
        bytes = bytes | (dot_y <<  8);
        bytes = bytes | dot_z;

        cache[i as usize].bits = bytes;
        println!("{:x}", bytes);
    }
}