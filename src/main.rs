// use std::error::Error;
// use std::fs::File;
// use std::collections::HashMap;

mod cpu;
mod memory;
pub use memory::Memory;

/*
#[derive(Deserialize, Debug)]
struct Opcode {
    code: u8,
    name: String,
    bytes: u8,
    clocks: i8
}

fn read_binary(file_name : &str) -> Vec<u8> {
   std::fs::read(file_name).unwrap_or(vec![])
}

// Create our lookup table to map code -> opcode info
// There's probably an easier way to do this
fn create_lookup_table(lst: Vec<Opcode>) -> HashMap<u8, Opcode> {
    let mut dict = HashMap::new();
    for op in lst {
        dict.insert(op.code, op);
    }

    dict
}

fn read_opcodes(file: &str) -> Result<Vec<Opcode>, Box<Error>> {
    let file = File::open(file)?;
    let opcodes = serde_json::from_reader(file)?;

    Ok(opcodes)
}
*/

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let fname: String = match args.get(1) {
        Some(v) => v.clone(),
        None    => String::from("")
    };

    if fname.len() == 0 {
        println!("Error: Need to give DMG file as command line argument!");
        return;
    }

    let mem = Memory::new(0x10000);
    cpu::run();

    /*
    let rom = read_binary(fname.as_str());
    let num_bytes = rom.len();
    let mut pc = 0;

    while pc < num_bytes {
        let op = &op_lookup[&rom[pc]];
        println!("{}: {}", pc, op.name);
        pc += 1;
    }
    */
}
