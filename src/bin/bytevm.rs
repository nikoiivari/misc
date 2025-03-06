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

// Type up a fake program in hexadcimal:
// echo -en "\xde\xad\xbe\xef\xde\xad\xbe\xe0" >> symbols.xe
// echo -en "\xde\xad\xbe\xef\xde\xad\xbe\xe1" >> symbols.xe
// echo -en "\xde\xad\xbe\xef\xde\xad\xbe\xe2" >> symbols.xe
// echo -en "\xff\xff\xff\xff\x00\x00\x00\x00" >> symbols.xe
// echo -en "\xff\xff\xff\xff\x00\x00\x00\x01" >> symbols.xe

// To dump 8 bytes per line as hexadecimal:
// od -A x -t x1z -w8 symbols.xe

// Bytecode virtual machine
// Read bytecode file and execute.

fn main () {
    // Cache of 4096 registers. (16 pages of 256 each). ~32KB cache
    let mut cache: [Reg; 4096] = [Reg {wbr: true, cap: false, bits: 0}; 4096];

    // Accum of 4096 registers. (16 pages of 256 each). ~32KB accum
    let mut _accum: [Reg; 4096];

    // header size in bytes
    let hdr = 16;

    // File loading and setup that an OS would do.
    let offset = 256; // 256 registers offset, to skip zero page
    // TODO: get filename from commandline!!!
    let buffer: Vec<u8> = fs::read("symbols.xe").unwrap();
    
    let mut codesize:u32 = buffer[4] as u32; //lowest byte
    let cs2b: u32 = buffer[3] as u32;
    let cs3b: u32 = buffer[2] as u32;    
    codesize = codesize | (cs2b << 8);
    codesize = codesize | (cs3b << 16); // 24bits of codesize
    println!("codesize: {:x}", codesize);
    
    let mut datasize:u32 = buffer[7] as u32; //lowest byte
    let ds2b: u32 = buffer[6] as u32;
    let ds3b: u32 = buffer[5] as u32;
    datasize = datasize | (ds2b << 8);
    datasize = datasize | (ds3b << 16); // 24bits of datasize (static data before heap)
    println!("datasize: {:x}", datasize);

    let mut mainsize:u32 = buffer[12] as u32; //lowest byte
    let ms2b: u32 = buffer[11] as u32;
    let ms3b: u32 = buffer[10] as u32;    
    mainsize = mainsize | (ms2b << 8);
    mainsize = mainsize | (ms3b << 16); // 24bits of numscope
    println!("mainsize: {:x}", mainsize);

    let mut entry:u32 = buffer[15] as u32; //lowest byte
    let e2b: u32 = buffer[14] as u32;
    let e3b: u32 = buffer[13] as u32;    
    entry = entry | (e2b << 8);
    entry = entry | (e3b << 16); // 24bits of entry
    println!("entry: {:x}", entry);

    // Put code into cache registers. Start from offset.
    for i in offset..(offset + codesize) {
        cache[i as usize].wbr = false;
        cache[i as usize].cap = false;
        
        // each of the 8 bytes
        let j:usize = (i - offset) as usize;
        let dot_s = buffer[hdr + (j * 8) + 0] as u64;
        let dot_t = buffer[hdr + (j * 8) + 1] as u64;
        let dot_u = buffer[hdr + (j * 8) + 2] as u64;
        let dot_v = buffer[hdr + (j * 8) + 3] as u64;
        let dot_w = buffer[hdr + (j * 8) + 4] as u64;
        let dot_x = buffer[hdr + (j * 8) + 5] as u64;
        let dot_y = buffer[hdr + (j * 8) + 6] as u64;
        let dot_z = buffer[hdr + (j * 8) + 7] as u64;

        let mut bytes: u64 = dot_s << 56;
        bytes = bytes | (dot_t << 48);
        bytes = bytes | (dot_u << 40);
        bytes = bytes | (dot_v << 32);
        bytes = bytes | (dot_w << 24);
        bytes = bytes | (dot_x << 16);
        bytes = bytes | (dot_y <<  8);
        bytes = bytes | dot_z;

        cache[i as usize].bits = bytes;
        println!("{:x}: {:x}", i, bytes);
    }

    for i in (offset + codesize)..(offset + codesize + datasize) {
        cache[i as usize].wbr = false;
        cache[i as usize].cap = false;

        // each of the 8 bytes
        let j:usize = (i - offset) as usize;
        let dot_s = buffer[hdr + (j * 8) + 0] as u64;
        let dot_t = buffer[hdr + (j * 8) + 1] as u64;
        let dot_u = buffer[hdr + (j * 8) + 2] as u64;
        let dot_v = buffer[hdr + (j * 8) + 3] as u64;
        let dot_w = buffer[hdr + (j * 8) + 4] as u64;
        let dot_x = buffer[hdr + (j * 8) + 5] as u64;
        let dot_y = buffer[hdr + (j * 8) + 6] as u64;
        let dot_z = buffer[hdr + (j * 8) + 7] as u64;

        let mut bytes: u64 = dot_s << 56;
        bytes = bytes | (dot_t << 48);
        bytes = bytes | (dot_u << 40);
        bytes = bytes | (dot_v << 32);
        bytes = bytes | (dot_w << 24);
        bytes = bytes | (dot_x << 16);
        bytes = bytes | (dot_y <<  8);
        bytes = bytes | dot_z;

        cache[i as usize].bits = bytes;
        println!("{:x}: {:x}", i, bytes);
    }
    
    // At this point code and initialized data is loaded in to cache. The next thing to do
    // is to set up scope main in the heap, zero it out, and also set WBR bit.

    for i in (offset + codesize + datasize)..(offset + codesize + datasize + mainsize) {
        cache[i as usize].wbr = true;
        cache[i as usize].cap = false;

        let dot_s = 0 as u64;
        let dot_t = 0 as u64;
        let dot_u = 0 as u64;
        let dot_v = 0 as u64;
        let dot_w = 0 as u64;
        let dot_x = 0 as u64;
        let dot_y = 0 as u64;
        let dot_z = 0 as u64;

        let mut bytes: u64 = dot_s << 56;
        bytes = bytes | (dot_t << 48);
        bytes = bytes | (dot_u << 40);
        bytes = bytes | (dot_v << 32);
        bytes = bytes | (dot_w << 24);
        bytes = bytes | (dot_x << 16);
        bytes = bytes | (dot_y <<  8);
        bytes = bytes | dot_z;

        cache[i as usize].bits = bytes;
        println!("{:x}: {:x}", i, bytes);
    }

}