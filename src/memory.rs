#![allow(dead_code)]

use std::fs;
use std::io;

pub struct Memory {
    mem:  Vec<u8>,
    rom:  Vec<u8>,
    bios: Vec<u8>
}

pub enum MemClient {
    CPU,
    PPU
}

impl Memory {
    pub fn new(size: usize) -> Memory {
        Memory {
            mem:  vec![0; size],
            rom:  Vec::new(),
            bios: Memory::default_bios()
        }
    }

    // TODO: Implement ROM switching and interfaces for different memory bank controllers.
    pub fn get(&self, addr: u16, _client: MemClient) -> u8 {
        let a = addr as usize;
        if a < 0x100 {
            if self.bootrom_enabled() {
                self.bios[a]
            } else {
                self.rom[a]
            }
        } else if a < 0x4000 {
            self.rom[a]
        } else if a < 0x8000 {
            println!("Reading from ROM bank N, this is unimplemented!");
            self.rom[a]
        } else {
            self.mem[a]
        }
    }

    pub fn set(&mut self, val: u8, addr: u16, _client: MemClient) {
        if addr >= 0xFF00 && addr <= 0xFF7F {
            // println!("[MEM] Setting I/O Register 0x{:04X} = 0x{:02X}", addr, val);
        }
        self.mem[addr as usize] = val;
    }

    pub fn load_rom_file(&mut self, file_name : &str) {
        self.rom = fs::read(file_name).unwrap_or(vec![])
    }

    pub fn load_bios_file(&mut self, file_name : &str) {
        self.bios = fs::read(file_name).unwrap_or(vec![])
    }

    fn default_bios() -> Vec<u8> {
        let mut bios = vec![0x0; 0x100];
        bios[0x01] = 0xC3; // Jump to the end of the boot ROM. (JP a16)
        bios[0x02] = 0xFC;
        bios[0xFC] = 0x3E; // Set A so we can use it to set Boot ROM. (LD A,d8)
        bios[0xFD] = 0x01;
        bios[0xFE] = 0xE0; // Disable Boot ROM with (0xFF50) = 1. (LDH (a8),A)
        bios[0xFF] = 0x50;
        bios
    }

    fn bootrom_enabled(&self) -> bool {
        self.mem[0xFF50] == 0
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
