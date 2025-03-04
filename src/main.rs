mod cpu;
mod memory;
mod display;
mod rom;

use std::io::Error;
use crate::cpu::Cpu;
use crate::memory::Memory;
use crate::rom::Rom;

fn main() {
    let mut cpu_processor = Cpu::new();
    let mut virtual_memory = Memory::new();
    let rom_result = Rom::load(String::from("chip8_bin/ibm.ch8"));
    let rom: Rom;

    match rom_result {
        Ok(r) => rom = r,
        Err(_) => panic!("Error occurred while loading rom!")
    }

    virtual_memory.load_rom(rom);
    println!("{:?}", virtual_memory);


    cpu_processor.run_bin(&virtual_memory);

}
