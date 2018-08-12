/*
TODO: Here we should handle rom-bank switching. When we specify an address between [0x4000, 0x8000)
      we need to return the data in the appropriate ROM bank. How do we know which one that is?
      I'm not sure yet. :)
*/

use std::fs;

pub struct Memory {
    mem:  Vec<u8>,
    rom:  Vec<u8>,
    bios: Vec<u8>
}

// TODO: Maybe implement an enum for our memory source?
// That way we can implement a lookup function that inputs an address and
// identifies which bank we need to load from. Ie. it would check if Boot ROM is
// enabled for < 0x100, or return the appropriate cartridge ID. Then we could
// implement our getter and setter functions to use this, that way we don't have
// to duplicate this identification process.

impl Memory {
    pub fn new(size: usize) -> Memory {
        Memory {
            mem:  vec![0; size],
            rom:  Vec::new(),
            bios: Memory::default_bios()
        }
    }

    // TODO: Need to pull from mem for some ranges, and ROM for others. Implement some functions
    // for ROM switching, so we can do that within the CPU.
    pub fn get(&self, addr: u16) -> u8 {
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

    pub fn set(&mut self, val: u8, addr: u16) {
        if addr >= 0xFF00 && addr <= 0xFF7F {
            println!("Setting I/O Register 0x{:04X} = 0x{:02X}", addr, val);
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
