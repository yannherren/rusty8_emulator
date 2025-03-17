use crate::rom::Rom;
use std::ops::Range;

const MAX_MEMORY_ADDRESS: usize = 4096;

#[derive(Debug)]
pub struct Memory {
    addressable_storage: [u8; MAX_MEMORY_ADDRESS],
    stack: [u16; 16],
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            addressable_storage: [0; MAX_MEMORY_ADDRESS],
            stack: [0; 16]
        }
    }

    pub fn load_rom(&mut self, rom: Rom) {
        let program_start_address = 0x200;

        for address in program_start_address..MAX_MEMORY_ADDRESS {
            let rom_offset = address - program_start_address;
            if rom_offset >= rom.content.len() {
                break;
            }
            self.addressable_storage[address] = rom.content[rom_offset];
        }
    }

    pub fn get_values_in_range(&self, address: Range<usize>) -> &[u8] {
        &self.addressable_storage[address]
    }
}