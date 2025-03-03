pub struct Cpu {
    timer_register: u8,
    sound_register: u8,
    stack_pointer: u8,
    program_counter: u16,
    gp_registers: [u8; 16],
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            timer_register: u8::MAX,
            sound_register: u8::MAX,
            stack_pointer: 0,
            program_counter: 0,
            gp_registers: [0; 16]
        }
    }

}