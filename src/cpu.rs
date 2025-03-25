use std::collections::HashMap;
use crate::memory::Memory;
use crate::register::Register;

const BIN_INSTRUCTION_START_ADDRESS: usize = 0x200;
const INSTRUCTION_SIZE_BYTES: usize = 2;

pub struct Cpu {
    timer_register: Register<u8>,
    sound_register: Register<u8>,
    carry_flag_register: Register<u8>,
    stack_pointer: u8,
    instruction_pointer: usize,
    gp_registers: HashMap<String, Register<u8>>,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            timer_register: Register::new(),
            sound_register: Register::new(),
            carry_flag_register: Register::new(),
            stack_pointer: 0,
            instruction_pointer: 0,
            gp_registers: HashMap::from([
                (String::from("V1"), Register::new()),
                (String::from("V2"), Register::new()),
                (String::from("V3"), Register::new()),
                (String::from("V4"), Register::new()),
                (String::from("V5"), Register::new()),
                (String::from("V6"), Register::new()),
                (String::from("V7"), Register::new()),
                (String::from("V8"), Register::new()),
                (String::from("V9"), Register::new()),
                (String::from("VA"), Register::new()),
                (String::from("VB"), Register::new()),
                (String::from("VC"), Register::new()),
                (String::from("VD"), Register::new()),
                (String::from("VE"), Register::new()),
                (String::from("VF"), Register::new()), // reserved for carry flag
            ])
        }
    }

    pub fn run_bin(&mut self, memory: &Memory) {
        self.instruction_pointer = BIN_INSTRUCTION_START_ADDRESS;
        let instruction = self.load_next_instruction(memory);
        self.run_from_instruction(instruction);
        // self.run_from_instruction(0x8236);
    }

    fn load_next_instruction(&mut self, memory: &Memory) -> u16 {
        let start_address = self.instruction_pointer;
        let end_address = start_address + INSTRUCTION_SIZE_BYTES;
        let slices = memory.get_values_in_range(start_address..end_address);
        let instruction_count = slices.len();
        let mut instruction: u16 = 0;
        for i in 0..instruction_count {
            let mut instruction_slice = slices[i] as u16;
            instruction_slice = instruction_slice << (instruction_count - i - 1);
            instruction += instruction_slice;
        }

        instruction
    }

    fn get_gp_reg(&mut self, reg_number: u16) -> &mut Register<u8> {
        self.gp_registers.entry(format!("V{}", reg_number.to_string())).or_insert(Register::new())
    }

    fn get_gp_reg_value(&self, reg_number: u16) -> u8 {
        self.gp_registers.get(&format!("V{}", reg_number.to_string())).unwrap_or(&Register::new()).value
    }

    fn run_from_instruction(&mut self, instruction: u16) {
        if instruction == 0 {
            return;
        }
        println!("{:#04x}", instruction);

        match instruction {
            0x0000..=0x0FFF => match instruction & 0xF0FF  {
                0x00EE => unimplemented!(),
                0x00E0 => unimplemented!("clear screen"),
                _ => unimplemented!()
            }
            0x1000..=0x1FFF => unimplemented!(),
            0x2000..=0x2FFF => unimplemented!(),
            0x3000..=0x3FFF => unimplemented!(),
            0x4000..=0x4FFF => unimplemented!(),
            0x5000..=0x5FFF if instruction & 0xF00F == 0x5000 => unimplemented!(),
            0x6000..=0x6FFF => unimplemented!(),
            0x7000..=0x7FFF => self.get_gp_reg(instruction & 0x0F00).add((instruction & 0x00F0) as u8),
            0x8000..=0x8FFF => match instruction & 0xF00F {
                0x8000 => {
                    let vy_value = self.get_gp_reg_value(instruction & 0x00F0);
                    let vx = self.get_gp_reg(instruction & 0x0F00);
                    vx.set(vy_value);
                },
                0x8001 => {
                    let vy_value = self.get_gp_reg_value(instruction & 0x00F0);
                    let vx = self.get_gp_reg(instruction & 0x0F00);
                    vx.or(vy_value);
                }
                0x8002 => {
                    let vy_value = self.get_gp_reg_value(instruction & 0x00F0);
                    let vx = self.get_gp_reg(instruction & 0x0F00);
                    vx.and(vy_value);
                },
                0x8003 => {
                    let vy_value = self.get_gp_reg_value(instruction & 0x00F0);
                    let vx = self.get_gp_reg(instruction & 0x0F00);
                    vx.xor(vy_value);
                },
                0x8004 => {
                    let vy_value = self.get_gp_reg_value(instruction & 0x00F0);
                    let vx = self.get_gp_reg(instruction & 0x0F00);
                    let cf_value = vx.add_with_carry(vy_value);
                    self.carry_flag_register.set(cf_value);
                },
                0x8005 => unimplemented!(),
                0x8006 => unimplemented!("Set Vx = Vx SHR 1"),
                0x8007 => unimplemented!(),
                0x800E => unimplemented!("here"),
                _ => {}
            }
            0x9000..=0x9FFF if instruction & 0xF00F == 0x9000 => unimplemented!(),
            0xA000..=0xAFFF => unimplemented!(),
            0xB000..=0xBFFF => unimplemented!(),
            0xC000..=0xCFFF => unimplemented!(),
            0xD000..=0xDFFF => unimplemented!(),
            0xE000..=0xEFFF => match instruction & 0xF0FF {
                0xE09E => unimplemented!(),
                0xE0A1 => unimplemented!(),
                _ => {}
            }
            0xF000..=0xFFFF => match instruction & 0xF0FF {
                0xF007 => unimplemented!(),
                0xF00A => unimplemented!(),
                0xF015 => unimplemented!(),
                0xF018 => unimplemented!(),
                0xF01E => unimplemented!(),
                0xF029 => unimplemented!(),
                0xF033 => unimplemented!(),
                0xF055 => unimplemented!(),
                0xF065 => unimplemented!(),
                _ => {}
            }
            _ => {}
        }
    }
}