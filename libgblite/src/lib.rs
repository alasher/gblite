pub mod memory;
pub mod cpu;
pub mod ppu;
pub mod util;
mod registers;
mod window;
mod lookup;

use std::collections::HashSet;

pub struct RuntimeConfig {
    pub rom_file: Option<String>,
    pub breakpoints: HashSet<u16>,
    pub killpoint: Option<u16>,
    pub dump_trace: bool,
    pub dump_mem: bool,
    pub verbose:  bool,
}

impl RuntimeConfig {
    pub fn new() -> Self {
        RuntimeConfig {
            rom_file: None,
            breakpoints: HashSet::new(),
            killpoint: None,
            dump_trace: false,
            dump_mem: false,
            verbose:  false,
        }
    }
}
