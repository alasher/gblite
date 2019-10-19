extern crate num;
extern crate ctrlc;
extern crate sdl2;
extern crate chrono;

mod registers;
mod cpu;
mod ppu;
mod window;
mod memory;
mod util;
mod lookup;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time;
use std::fs;
use chrono::{Utc, Datelike, Timelike};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let fname: String = match args.get(args.len()-1) {
        Some(v) => v.clone(),
        None    => String::from("")
    };

    let file_metadata = fs::metadata(&fname);
    if  args.len() < 2 || !file_metadata.unwrap().is_file() {
        println!("Error: Need to give DMG file as command line argument!");
        println!("Option -d: Dump system memory to a log file upon termination.");
        return;
    } else {
        println!("Opening ROM file: \"{}\"", fname);
    }

    // TODO: There's gotta be a cleaner way to read command line options
    let dump_mem = match args.get(1) {
        Some(v) => (v == "-d"),
        None => false
    };

    // Register Ctrl-C handling
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    let mut mem = memory::Memory::new(0x10000);
    mem.load_rom_file(&fname);
    let mem = Arc::new(Mutex::new(mem));

    let ppu = ppu::PPU::new(mem.clone());
    let mut z80 = cpu::CPU::new(mem.clone(), ppu);
    // let mut cnt = 0;

    // Now, run instructions *literally* forever!
    loop {
        if !running.load(Ordering::SeqCst) {
            println!("Received Ctrl+C signal, exiting!");
            break;
        }

        if !z80.tick() { break; }
        // cnt += 1;

        // if cfg!(debug_assertions) {
        //     if (cnt % 1000) == 0 {
        //         println!("Instruction count: {}", cnt);
        //     }
        // }
    }

    if dump_mem {
        let dt = Utc::now();
        let fname = format!("gblite_mem_{}_{:02}_{:02}_{}.log", dt.year(), dt.month(), dt.day(),
                            dt.num_seconds_from_midnight());
        let mref = mem.lock().unwrap();
        match (*mref).dump_to_file(&fname) {
            Ok(_r) => (),
            Err(e) => panic!("Error dumping memory: {}", e),
        }
    }

    thread::sleep(time::Duration::from_millis(100));
}
