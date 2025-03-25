#![allow(dead_code, unused_imports)]

use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Write;

#[derive(Debug)]
struct OpRow {
    oprow: u64,
    state: u64,
}

impl OpRow {
    pub fn new(oprow:u64, state:u64) -> Self {
        OpRow {
            oprow: oprow,
            state: state,
        }
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
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
    Idhexym,
    Idmyxeh,
    // Operations
    IdAssignAccumToVar,
    IdAssignVarToAccum,
    IdAssignCacheToVar,
    IdAssignVarToCache,
}

// ask, req
#[derive(Debug)]
enum MemType {
    MemNotAMem,
    MemAccumPages,
    MemCachePages,
    MemAccumRegs,
    MemCacheRegs,
}

// var, in, out
#[derive(Debug)]
enum PackType {
    PackNotAPack,
    PackPack,
    PackEven,
    PackOdd,
}

#[derive(Debug)]
enum TypeType {
    TypeNotAType,
    TypeCapability,
    TypeSigned,
    TypeUnsigned,
}

// All identifiers and their parameters
#[derive(Debug)]
struct Id {
    it:IdType,
    param: String,
    // ask, req
    mt: MemType,
    mpages: u16,
    mregs: u8,
    // var, in, out
    pt: PackType,
    swizzles: u8,
    size: u8,
    tt: TypeType,
    // ...
}

impl Id {
    pub fn new(it: IdType, param: String) -> Self {
        Id {
            it: it,
            param: param,
            // ask, req
            mt: MemType::MemNotAMem,
            mpages: 0x0,
            mregs: 0x0,
            // var, in, out
            pt: PackType::PackNotAPack,
            swizzles: 0x0,
            size: 0x0,
            tt: TypeType::TypeNotAType,
        }
    }
}

struct ParserState {
    in_hexym: bool,
    in_scope: bool,
    in_fun: bool,
}

impl ParserState {
    pub fn new(in_hexym: bool, in_scope: bool, in_fun: bool) -> Self {
        ParserState {
            in_hexym: in_hexym,
            in_scope: in_scope,
            in_fun: in_fun,
        }
    }
}

//====

//====
fn nand(ina:u64, inb:u64) -> u64
{
    return !(ina & inb);
}
//====

fn main ()
{
    // Vector of Ids
    let mut ids: Vec<Id> = vec![];
    let mut funstack: Vec<u32> = vec![];
    let mut varinoutstack: Vec<u32> = vec![];
    let mut ps: ParserState = ParserState::new(false, false, false);
    
    let mut infilepath: &str = "bin.out";

    let mut ob = Vec::<u8>::new(); //Out buffer ob
    
    //commandline args
    let args: Vec<String> = env::args().collect();
    if 1 < args.len()
    {
        //println!("{:?}", args);
        infilepath = &args[1];
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

            // parse code
            if "" != code {
                let _o:OpRow; let i:Id;
                (_o, i ) = parse_code(code, &ids, &funstack, &varinoutstack, &mut ps);
                //println!("{:?}, {:?}", o, i);
                if IdType::Idfun == i.it {                    
                    ids.push(i);
                    let funindex:u32 = (ids.len() - 1) as u32;
                    funstack.push(funindex);
                    println!("funstack: {:?}", funindex);
                } else if IdType::Idnuf == i.it {
                    ids.push(i);
                    funstack.pop();
                } else if IdType::Idvar == i.it || IdType::Idin == i.it || IdType::Idout == i.it {
                    ids.push(i);
                    let varinoutindex:u32 = (ids.len() - 1) as u32;
                    varinoutstack.push(varinoutindex);
                    println!("varinoutstack: {:?}", varinoutindex);
                } else {
                    ids.push(i);
                }
            } // else an empty code; ignore empty code
        }
    }

    // ==== Write executable file ====

    write_xe_header(&mut ob, 0x000003, 0x000002, 0x000001, 0x00000);
    
    // strip file extension from infilepath
    let pathparts: Vec<&str> = infilepath.split('.').collect();
    let outfilepath = pathparts[0];
    let outfileext = outfilepath.to_owned() + ".xe";
    println!("Writing executable: {:?}", outfileext);
    let mut outfile = File::create(outfileext).unwrap();
    outfile.write(&ob).unwrap();
}

// parse_code -- generate Op struct for code statement
fn parse_code ( code:String,
                ids:&Vec<Id>,
                funstack:&Vec<u32>,
                _varinoutstack:&Vec<u32>,
                ps:&mut ParserState ) -> (OpRow, Id)
{
    
    let mut o: OpRow = OpRow::new(0x0, 0x0);
    let mut i: Id = Id {
        it: IdType::IdNotAnId,
        param: "".to_string(),
        // ask, req
        mt: MemType::MemNotAMem,
        mpages: 0x0,
        mregs: 0x0,
        // var, in, out
        pt: PackType::PackNotAPack,
        swizzles: 0x0,
        size: 0x0,
        tt: TypeType::TypeNotAType,
    };

    let v: Vec<&str> = code.split_whitespace().collect(); // does this work with tabs?
    if 0 < v.len() {
        //println!("{:?}", v[0]);
        match v[0] {
            "ask" => i = parse_id_mem(v, IdType::Idask),
            "req" => i = parse_id_mem(v, IdType::Idreq),
            "use" => i = parse_id_use(v),
            "enum" => println!("enum begins..."),
            "mune" => println!("enum ends."),
            "struc" => println!("struc begins..."),
            "curts" => println!("struc ends."),
            "scope" => i = parse_id_scope(v),
            "epocs" => i.it = IdType::Idepocs,                       
            "var" => i = parse_id_var(v, IdType::Idvar),
            "in"  => i = parse_id_var(v, IdType::Idin),
            "out" => i = parse_id_var(v, IdType::Idout),
            "fun" => i = parse_id_fun(v),
            "return" => println!("function returns before completion..."),
            "nuf" => i = parse_id_nuf(v, ids, funstack),
            "hexym" => i = parse_id_hexym(v, ps),
            "myxeh" => i = parse_id_myxeh(v, ps),
            "case" => println!("case sth..."),
            "esac" => println!("case ends."),
            "fit" => println!("fit sth..."),
            "tif" => println!("fit ends."),
            "if"  => println!("if sth..."),
            "then" => println!("if then..."),
            "else" => println!("if else..."),
            "fi"  => println!("if ends."),
            &_ => o = parse_op_line(v, ps), //TODO: This needs to know if it is in a hexym block.
        }
    }
    
    (o, i)
}

fn parse_id_mem (v: Vec<&str>, idt: IdType)-> Id {
    if 2 != v.len() {
        println!("Error: identifier expects one parameter.");
        return Id::new(IdType::IdNotAnId, "".to_string());
    }
    //println!("{:?}", v);
    //TODO: parse memory parameter!!
    let i:Id = Id::new(idt, v[1].to_string()); //TODO: parse memory!!
    i
}

fn parse_id_use (v: Vec<&str>) -> Id {
    if 2 != v.len() {
        println!("Error: Identifier use expects one parameter");
        return Id::new(IdType::IdNotAnId, "".to_string());
    }
    //println!("{:?}", v); 
    let i:Id = Id::new(IdType::Iduse, v[1].to_string());
    i
}

fn parse_id_scope (v: Vec<&str>) -> Id {
    if 2 != v.len() {
        println!("Error: Identifier scope expects one parameter");
        return Id::new(IdType::IdNotAnId, "".to_string());
    }
    println!("{:?}", v);
    let i:Id = Id::new(IdType::Idscope, v[1].to_string());
    i
}

fn parse_id_var (v: Vec<&str>, idt: IdType) -> Id {
    if 6 != v.len() {
        println!("Error: Identifier expects five parameters.");
        return Id::new(IdType::IdNotAnId, "".to_string());
    }
    let ctrimmed = v[1].trim_end_matches(':');
    println!("{:?}", v);
    let mut i:Id = Id::new(idt, ctrimmed.to_string());
    
    match v[2] {
        "even" => i.pt = PackType::PackEven,
        "odd"  => i.pt = PackType::PackOdd,
        "pack" => i.pt = PackType::PackPack,
        &_ => println!("Error: not a PackType."),
    }

    // TODO: parse swizzle!!
    //match v[3] {

    //}

    match v[4] {
        "8byte" => i.size = 8,
        "7byte" => i.size = 7,
        "6byte" => i.size = 6,
        "5byte" => i.size = 5,
        "4byte" => i.size = 4,
        "3byte" => i.size = 3,
        "2byte" => i.size = 2,
        "1byte" => i.size = 1,
        &_ => println!("Error: not a legitimate size."),
    }
    
    match v[5] {
        "capability"    => i.tt = TypeType::TypeCapability,
        "signed"        => i.tt = TypeType::TypeSigned,
        "unsigned"      => i.tt = TypeType::TypeUnsigned,
        &_ => println!("Error: not a TypeType."),
    }
    //println!("{:?}", i);
    i
}

fn parse_id_fun (v:Vec<&str>) -> Id {
    println!("{:?}", v);
    // TODO: check actual number of function parameters inside tuple    
    // TODO: actually parse fun parameters!! Now builds fun without initializing parameters.

    let i:Id = Id::new(IdType::Idfun, v[1].to_string());
    //println!("{:?}", i);
    i
}

fn parse_id_nuf (v:Vec<&str>, ids:&Vec<Id>, funstack:&Vec<u32>) -> Id {
    println!("{:?}", v);
    // TODO: actually parse return values inside tuple!!!
    
    let funindex = funstack.len() - 1;
    let i:Id = Id::new(IdType::Idnuf, ids[(funstack[funindex]) as usize].param.to_string());
    //println!("{:?}", i);
    i
}

//TODO: actually parse tuple!!
//TODO: parsing tuples is needed for taking fun parameters
//fn parse_id_tuple () -> Id {}

fn parse_id_hexym (v:Vec<&str>, ps:&mut ParserState) -> Id {
    println!("{:?}", v);
    ps.in_hexym = true;
    let i:Id = Id::new(IdType::Idhexym, "".to_string());
    i
}

fn parse_id_myxeh (v:Vec<&str>, ps:&mut ParserState) -> Id {
    println!("{:?}", v);
    ps.in_hexym = false;
    let i:Id = Id::new(IdType::Idmyxeh, "".to_string());
    i
}

//TODO: actually read hexym values!!!
fn parse_hexym_line(v: Vec<&str>) -> OpRow {
    println!("{:?}", v);
    // read byte from hexadecimal
    let s:u8 = parse_hexym_byte(v[0]);
    let t:u8 = parse_hexym_byte(v[1]);
    let u:u8 = parse_hexym_byte(v[2]);
    let v:u8 = parse_hexym_byte(v[3]);
    //let w:u8 = parse_hexym_byte(v[4]); //FIXME wierd!
    //let x:u8 = parse_hexym_byte(v[5]);
    //let y:u8 = parse_hexym_byte(v[6]);
    //let z:u8 = parse_hexym_byte(v[7]);
    
    //TODO: pack together in a u64
    println!("{:?}", s);
    let o = OpRow::new(0x0, 0x0);
    o
}

fn parse_hexym_byte(s: &str) -> u8 {
    if 2 != s.len() { println!("Error: Hexym byte is to be exactly two characters."); }
    let chvec: Vec<char> = s.chars().collect();
    let ch0:char = chvec[0];
    let ch1:char = chvec[1];
    if !ch0.is_ascii_hexdigit() { println!("Error: Not a hexdigit."); }
    let bhi:u8 = ch0.to_digit(16).unwrap() as u8;
    if !ch1.is_ascii_hexdigit() { println!("Errot: Not a hexdigit."); }
    let blo:u8 = ch1.to_digit(16).unwrap() as u8;
    let b: u8 = (bhi << 4) | blo;
    b
}

fn parse_op_line(v: Vec<&str>, ps: &ParserState) -> OpRow {
    // Op can be a hexym row or an assignment
    // Figure out which one it is:

    // Find out if in a hexym block.
    if true == ps.in_hexym { 
        let o:OpRow = parse_hexym_line(v);
        return o
    } else { // Not in a hexym block
        let o = OpRow::new(0x0, 0x0);
        return o
    }
}

//==== Writing the executable ====

fn write_xe_header (ob: &mut Vec<u8>, codesize: u32, datasize: u32, numscope: u32, entry: u32) {
    // XE MAGIC
    ob.push(0x58);
    ob.push(0x45);
        
    // code size
    ob.push(((codesize & 0b00000000111111110000000000000000) >> 16) as u8);
    ob.push(((codesize & 0b00000000000000001111111100000000) >> 8) as u8);
    ob.push(((codesize & 0b00000000000000000000000011111111)) as u8);
    // data size
    ob.push(((datasize & 0b00000000111111110000000000000000) >> 16) as u8);
    ob.push(((datasize & 0b00000000000000001111111100000000) >> 8) as u8);
    ob.push(((datasize & 0b00000000000000000000000011111111)) as u8);

    // ST
    ob.push(0x53);
    ob.push(0x54);

    // numscope
    ob.push(((numscope & 0b00000000111111110000000000000000) >> 16) as u8);
    ob.push(((numscope & 0b00000000000000001111111100000000) >> 8) as u8);
    ob.push(((numscope & 0b00000000000000000000000011111111)) as u8);
    // code entry point
    ob.push(((entry & 0b00000000111111110000000000000000) >> 16) as u8);
    ob.push(((entry & 0b00000000000000001111111100000000) >> 8) as u8);
    ob.push(((entry & 0b00000000000000000000000011111111)) as u8);
}


//TODO: actually write 
//fn write_exe_code (ob: &mut Vec<u8>, codesize: u32, ) {

//}