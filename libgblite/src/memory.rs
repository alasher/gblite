#![allow(dead_code)]

use std::fs;
use std::io;

pub struct Memory {
    mem:  Vec<u8>,
    rom:  Vec<u8>
}

pub enum MemClient {
    CPU,
    PPU
}

impl Memory {
    pub fn new(size: usize) -> Memory {

        let mut v = vec![0; size];
        v[0xff50] = 1;

        Memory {
            mem:  v,
            rom:  Vec::new()
        }
    }

    // TODO: Implement ROM switching and interfaces for different memory bank controllers.
    pub fn get(&self, addr: u16, _client: MemClient) -> u8 {
        let a = addr as usize;
        if a < 0x4000 {
            self.rom[a]
        } else if a < 0x8000 {
            self.rom[a]
        } else {
            self.mem[a]
        }
    }

    pub fn set(&mut self, val: u8, addr: u16, _client: MemClient) {
        let a = addr as usize;
        if a < 0x4000 {
            self.rom[a] = val;
        } else if a < 0x8000 {
            self.rom[a] = val;
        } else {
            self.mem[a] = val;
        }
    }

    pub fn load_rom_file(&mut self, file_name : &str) {
        self.rom = fs::read(file_name).unwrap_or(vec![])
    }

    // For debug use only: do a hex dump of the contents of our ROM cartridge.
    fn generate_dump(&self, is_rom: bool) -> String {
        let mut dump = String::new();
        let row_len = 32;
        let mem_src = if is_rom { &self.rom } else { &self.mem };

        for (i, byte) in mem_src.iter().enumerate() {
            if i % row_len == 0 {
                dump = format!("{}0x{:04x}:  ", dump, i);
            }

            dump = format!("{}{:02x}", dump, byte);

            if (i+1) % 32 == 0 {
                dump = format!("{}\n", dump);
            } else {
                dump = format!("{} ", dump);
            }
        }

        dump
    }

    pub fn dump_rom_to_file(&self, file_name: &str) -> io::Result<()> {
        println!("Dumping to file \"{}\"...", file_name);
        let mem_dump = self.generate_dump(true);
        fs::write(file_name, mem_dump)?;
        Ok(())
    }

    pub fn dump_rom(&self) {
        let mem_dump = self.generate_dump(true);
        print!("{}", mem_dump);
    }

    pub fn dump_to_file(&self, file_name: &str) -> io::Result<()> {
        println!("Dumping to file \"{}\"...", file_name);
        let mem_dump = self.generate_dump(false);
        fs::write(file_name, mem_dump)?;
        Ok(())
    }

    pub fn dump(&self) {
        let mem_dump = self.generate_dump(false);
        print!("{}", mem_dump);
    }
}
