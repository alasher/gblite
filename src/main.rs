mod cpu;
mod memory;
mod util;

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

    let mut mem = memory::Memory::new(0x10000);
    mem.load_rom_file(&fname);

    let mut z80 = cpu::CPU::new(mem);
    let mut last_pc = z80.get_pc();
    let mut loop_cnt = 0;

    // Now, run instructions *literally* forever!
    loop {
        if !z80.process() { break; }
        if z80.get_pc() == last_pc {
            loop_cnt += 1;
        } else {
            last_pc = z80.get_pc();
        }
        if loop_cnt >= 20 {
            // Just a really simple loop detector, since apparently Ctrl+C isn't a thing in Rust?
            println!("Fatal error: stuck in a single-instruction loop.");
            break;
        }

    }
}
