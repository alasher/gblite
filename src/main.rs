mod cpu;
mod memory;

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

    // Now, run instructions *literally* forever!
    loop {
        if !z80.process() { break; }
    }
}
