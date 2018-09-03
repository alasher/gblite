extern crate num;
extern crate ctrlc;

mod registers;
mod cpu;
mod memory;
mod util;
mod lookup;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time;

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

    // Register Ctrl-C handling
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    let mut mem = memory::Memory::new(0x10000);
    mem.load_rom_file(&fname);

    let mut z80 = cpu::CPU::new(mem);
    let mut cnt = 0;

    // Now, run instructions *literally* forever!
    loop {
        if !running.load(Ordering::SeqCst) { 
            println!("Received Ctrl+C signal, exiting!");
            break;
        }

        if !z80.process() { break; }
        cnt += 1;

        if cfg!(debug_assertions) {
            thread::sleep(time::Duration::from_millis(1));
            if (cnt % 1000) == 0 {
                println!("Instruction count: {}", cnt);
            }
        }
    }
}
