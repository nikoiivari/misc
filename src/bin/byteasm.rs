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
enum IdType {
    IdNotAnId,
    Idask,
    Idreq,
    Iduse,
    Idscope,
    Idepocs,
    Idvar,
    Idin,
    Idout,
    Idfun,
    Idnuf,
}

#[derive(Debug)]
enum MemType {
    MemNotAMem,
    MemAccumPages,
    MemCachePages,
    MemAccumRegs,
    MemCacheRegs,
}

#[derive(Debug)]
struct Id {
    t:IdType,
    param: String,
    m: MemType,
}

impl Id {
    pub fn new(t:IdType, param:String, m:MemType) -> Self {
        Id {
            t: t,
            param: param,
            m: m,
        }
    }
}

//====

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
                let _o:Op; let _i:Id;
                (_o, _i) = parse_code(code);
                //println!("{:?}, {:?}", o, i);
            } // else an empty code; ignore empty code
        }
    }
}

// parse_code -- generate Op struct for code statement
fn parse_code (code:String) -> (Op, Id) {
    
    let _o: Op = Op::new(0x0, 0x0);
    let mut i: Id = Id::new(IdType::IdNotAnId, "foo".to_string(), MemType::MemNotAMem);

    let v: Vec<&str> = code.split(' ').collect(); // does this work with tabs?
    if 0 < v.len() {
        //println!("{:?}", v[0]);
        match v[0] {
            "ask" => i = parse_id(v, IdType::Idask),
            "req" => i = parse_id(v, IdType::Idreq),
            "use" => println!("use module..."),
            "enum" => println!("enum begins..."),
            "mune" => println!("enum ends."),
            "struc" => println!("struc begins..."),
            "curts" => println!("struc ends."),
            "scope" => println!("scope begins..."),
            "epocs" => println!("scope ends."),
            "var" => println!("variable..."),
            "in"  => println!("function input..."),
            "out" => println!("function output..."),
            "fun" => println!("function declaration..."),
            "return" => println!("function returns before completion..."),
            "nuf" => println!("function declaration ends."),
            "case" => println!("case sth..."),
            "esac" => println!("case ends."),
            "fit" => println!("fit sth..."),
            "tif" => println!("fit ends."),
            "if"  => println!("if sth..."),
            "then" => println!("if then..."),
            "else" => println!("if else..."),
            "fi"  => println!("if ends."),
            &_ => println!("something else."),
        }
    }
    
    (_o, i)
}

fn parse_id ( v: Vec<&str>, idt: IdType)-> Id {
    let mut i: Id = Id::new(IdType::IdNotAnId, "foo".to_string(), MemType::MemNotAMem);
    if 1 < v.len() {
        println!("{:?}", v);
        //match v[1] {
            
        //}
    }
    else {
        println!("Error: identifier expects parameter.");
    }
    i
}
