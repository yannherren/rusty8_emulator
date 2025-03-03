mod cpu;
mod memory;
mod display;
mod rom;

use crate::cpu::Cpu;
use crate::memory::Memory;
use crate::rom::Rom;

fn main() {
    let cpu_processor = Cpu::new();
    let virtual_memory = Memory::new();
    let loaded_rom = Rom::load(String::from("ibm.ch8"));


}
