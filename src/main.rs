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

pub struct RuntimeConfig {
    rom_file: Option<String>,
    dump_mem: bool,
    verbose:  bool,
}

impl RuntimeConfig {
    pub fn new() -> Self {
        RuntimeConfig {
            rom_file: None,
            dump_mem: false,
            verbose:  false,
        }
    }
}

fn print_help_and_exit() {
    println!("{} version v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    println!("Option -d: Dump system memory to a log file upon termination.");
    println!("Option -v: Enable verbose instruction execution output.");
    std::process::exit(1);
}

fn main() {
    let mut cfg: RuntimeConfig = RuntimeConfig::new();

    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "-d" => { cfg.dump_mem = true; },
            "-v" => { cfg.verbose  = true; },
            other => {
                if &other[0..1] != "-" {
                    cfg.rom_file = Some(arg.clone());
                } else {
                    eprintln!("Read invalid argument, {}\n", other);
                    print_help_and_exit();
                }
            },
        }
    }

    let fname = match &cfg.rom_file {
        Some(f) => f,
        None => {
            print_help_and_exit();
            unreachable!();
        }
    };

    match fs::metadata(&fname) {
        Ok(meta) => {
            if !meta.is_file() { print_help_and_exit(); }
        },
        Err(e) => {
            eprintln!("Error reading file: {}\n", e);
            print_help_and_exit();
        }
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
    let mut z80 = cpu::CPU::new(mem.clone(), ppu, &cfg);
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

    if cfg.dump_mem {
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
