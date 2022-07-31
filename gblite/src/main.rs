use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time;
use std::fs;

use libgblite::memory::Memory;
use libgblite::cpu::CPU;
use libgblite::ppu::PPU;
use libgblite::util::create_file_name;

fn print_help_and_exit() {
    println!("{} version v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    println!("Option -d: Dump system memory to a log file upon termination.");
    println!("Option -b [address]: Break at the given PC address. Can be specified multiple times.");
    println!("Option -k [address]: Kill the program at the given PC address. Can only be specified once.");
    println!("Option -t: Log all instruction output to a trace file.");
    println!("Option -v: Enable verbose instruction execution output.");
    std::process::exit(1);
}

fn main() {
    let mut cfg = libgblite::RuntimeConfig::new();
    let mut arg_skip = 0;
    let mut arg_id = 1;

    for arg in std::env::args().skip(1) {
        if arg_skip > 0 {
            arg_skip -= 1;
        } else {
            match arg.as_str() {
                "-d" => { cfg.dump_mem = true; },
                "-b" => {
                    arg_skip = 1;
                    let addr_str = std::env::args().nth(arg_id+1).unwrap();
                    let addr_str = addr_str.trim_start_matches("0x");
                    match u16::from_str_radix(addr_str, 16) {
                        Ok(addr) => {   println!("Parsed as: {}", addr);
                                        cfg.breakpoints.insert(addr); },
                        Err(e) => { println!("Error parsing breakpoint argument \"{}\": {}", addr_str, e); },
                    }
                },
                "-k" => {
                    arg_skip = 1;
                    let addr_str = std::env::args().nth(arg_id+1).unwrap();
                    let addr_str = addr_str.trim_start_matches("0x");
                    match u16::from_str_radix(addr_str, 16) {
                        Ok(addr) => { cfg.killpoint = Some(addr); },
                        Err(e) => { println!("Error parsing breakpoint argument \"{}\": {}", addr_str, e); },
                    }
                },
                "-t" => { cfg.dump_trace = true; },
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

        arg_id += 1;
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

    let mut mem = Memory::new(0x10000);
    mem.load_rom_file(&fname);
    let mem = Arc::new(Mutex::new(mem));

    let ppu = PPU::new(mem.clone());
    let mut z80 = CPU::new(mem.clone(), ppu, &cfg);

    // Run instructions until the end of time
    loop {
        if !running.load(Ordering::SeqCst) {
            println!("Received Ctrl+C signal, exiting!");
            break;
        }

        if !z80.tick() { break; }
    }

    if cfg.dump_mem {
        let fname = create_file_name("_mem");
        let mref = mem.lock().unwrap();
        match (*mref).dump_to_file(&fname) {
            Ok(_r) => (),
            Err(e) => panic!("Error dumping memory: {}", e),
        }
    }

    thread::sleep(time::Duration::from_millis(100));
}
