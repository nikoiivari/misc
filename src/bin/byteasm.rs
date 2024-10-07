#![allow(dead_code, unused_imports)]

use std::env;
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
struct Id {
    id:String,
}

impl Id {
    pub fn new(id:String) -> Self {
        Id {
            id: id,
        }
    }
}

fn nand(ina:u64, inb:u64) -> u64
{
    return !(ina & inb);
}

fn main ()
{
    
    //commandline args
    let args: Vec<String> = env::args().collect();
    if 1 < args.len()
    {
        //println!("{:?}", args);
        let infilepath = &args[1];
        let mut infile = File::open(infilepath).unwrap();

        let mut s: String = Default::default();
        let _ = infile.read_to_string(&mut s);

        for line in s.lines() {
            let code:String;
            // separate code from a posible comment at the end of line     
            if line.contains('#') {
                let (line_untrimmed, _comment) = line.split_once('#').unwrap();
                let line_trimmed = line_untrimmed.trim();
                code = line_trimmed.to_string();
            } else {
                let line_trimmed = line.trim();
                code = line_trimmed.to_string();
            }

            // parse instruction
            if "" != code {
                let o:Op; let i:Id;
                (o, i) = parse_code(code);
                //println!("{:?}, {:?}", o, i);
            } // else an empty code; ignore empty code
        }
    }
}

// parse_code -- generate Op struct for code statement
fn parse_code (code:String) -> (Op, Id) {
    
    
    let v: Vec<&str> = code.split(' ').collect(); // does this work with tabs?
    if 0 < v.len() {
        //println!("{:?}", v[0]);
        match v[0] {
            "ask" => println!("ask for..."),
            "req" => println!("require..."),
            "var" => println!("variable..."),
            "in"  => println!("function input..."),
            "out" => println!("function output..."),
            "fun" => println!("function declaration..."),
            "return" => println!("function return..."),
            &_ => println!("something else."),
        }
    }
    
    let o:Op = Op::new(0x0, 0x0);
    let i:Id = Id::new("foo".to_string());
    (o, i)
}