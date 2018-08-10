/*
TODO: Here we should handle rom-bank switching. When we specify an address between [0x4000, 0x8000)
      we need to return the data in the appropriate ROM bank. How do we know which one that is?
      I'm not sure yet. :)
*/

use std::fs;

pub struct Memory {
    mem: Vec<u8>,
    rom: Vec<u8>
}

impl Memory {
    pub fn new(size: usize) -> Memory {
        Memory {
            mem: vec![0; size],
            rom: Vec::new()
        }
    }

    // TODO: Need to pull from mem for some ranges, and ROM for others. Implement some functions
    // for ROM switching, so we can do that within the CPU.
    pub fn get(&self, addr: usize) -> u8 {
        if addr < 0x4000 {
            self.rom[addr]
        } else if addr < 0x8000 {
            println!("Reading from ROM bank N, this is unimplemented!");
            self.rom[addr]
        } else {
            self.mem[addr]
        }
    }

    pub fn set(&mut self, val: u8, addr: usize) {
        self.mem[addr] = val;
    }

    pub fn load_rom_file(&mut self, file_name : &str) {
       self.rom = fs::read(file_name).unwrap_or(vec![])
    }

    // For debug use only: do a hex dump of the contents of our ROM cartridge.
    pub fn dump_rom(&self) {
        let row_len = 32;
        for (i, byte) in self.rom.iter().enumerate() {

            // TODO: Pad the address by log16 of the maximum address we need to print
            if i % row_len == 0 {
                print!("0x{:04x}:  ", i);
            }

            print!("{:02x}", byte);

            if (i+1) % 32 == 0 {
                println!("");
            } else {
                print!(" ");
            }
        }

        println!("ROM size is {} bytes", self.rom.len());
    }

}
