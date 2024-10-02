#![allow(dead_code, unused_imports)]

use std::env;
//use std::path::Path;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
struct Op {
        opcode:u16,
          dest:u32,
}

impl Op {
    pub fn new(opcode:u16, dest:u32) -> Self {
        Op {
            opcode: opcode,
              dest: dest,
        }
    }
}

#[derive(Debug)]
struct Label {
    label:String,
}

impl Label {
    pub fn new(label:String) -> Self {
        Label {
            label: label,
        }
    }
}

fn nand(ina:u64, inb:u64) -> u64
{
    return !(ina & inb);
}

fn main ()
{
    //let rval:u64 = nand(0b1111, 0b1111);
    //println!("nand(0b1111, 0b1111) = {rval:#b}");
    
    //commandline args
    let args: Vec<String> = env::args().collect();
    if 1 < args.len()
    {
        //println!("{:?}", args);
        let infilepath = &args[1];
        let mut infile = File::open(infilepath).unwrap();

        //let reader = BufReader::new(infile);
        let mut s: String = Default::default();
        let _ = infile.read_to_string(&mut s);

        for line in s.lines() {
            //println!("{:?}", line);
            let code:String;
            // separate code from a posible comment at the end of line     
            if line.contains('#') {
                let (statement_untrimmed, _comment) = line.split_once('#').unwrap();
                let statement = statement_untrimmed.trim();
                code = statement.to_string();
                println!("{:?}", code);
            } else {
                let line_trimmed = line.trim();
                code = line_trimmed.to_string();
                println!("{:?}", code);
            }

            // parse instruction
            if "" != code {
                let o:Op; let l:Label;
                (o, l) = parse_code(code);
                println!("{:?}, {:?}", o, l);
            } // else an empty code -- ignore empty code
        }
        //println!("{}", s);
    }
}

// parse_code -- generate Op struct for code statement
fn parse_code (code:String) -> (Op, Label ) {
    //println!("{:?}", code);
    // parse instruction here ...
    let v: Vec<&str> = code.split(' ').collect(); // does this work with tabs?
    if 0 < v.len() {
        //println!("{:?}", v[0]);
        //match v[0] {
            //"foo" =>
        //}
    }
    
    let o:Op = Op::new(0x0, 0x0);
    let l:Label = Label::new("foo".to_string());
    (o, l)
}