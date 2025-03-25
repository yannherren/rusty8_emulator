use num_traits::{one, zero, CheckedAdd, Unsigned, WrappingAdd};
use std::ops::{BitAnd, BitOr, BitXor};

pub trait UnsignedNum:
    Copy
    + Unsigned
    + BitOr<Output = Self>
    + BitXor<Output = Self>
    + BitAnd<Output = Self>
    + CheckedAdd<Output = Self>
    + WrappingAdd<Output = Self> {}

impl UnsignedNum for u8 {}
impl UnsignedNum for u16 {}
impl UnsignedNum for u32 {}
impl UnsignedNum for u64 {}
impl UnsignedNum for u128 {}

pub struct Register<T: UnsignedNum> {
    pub value: T
}

impl<T: UnsignedNum> Register<T> {

    pub fn new() -> Self {
        Register {
            value: T::zero()
        }
    }

    pub fn set(&mut self, new_value: T) {
        self.value = new_value;
    }

    pub fn get(&mut self) -> T {
        self.value
    }

    pub fn set_from_reg(&mut self, reg: &Register<T>) {
        self.value = (*reg).value;
    }

    pub fn add(&mut self, new_value: T) {
        self.value = self.value + new_value;
    }

    pub fn subtract(&mut self, reg: &Register<T>) {
        self.value = self.value - reg.value;
    }

    pub fn or(&mut self, value: T) {
        self.value = self.value | value;
    }

    pub fn and(&mut self, value: T) {
        self.value = self.value & value;
    }

    pub fn xor(&mut self, value: T) {
        self.value = self.value ^ value;
    }

    pub fn add_with_carry(&mut self, value: T) -> T {
        let carry = self.value.checked_add(&value).is_none();
        self.value = self.value.wrapping_add(&value);
        if carry {zero()} else { one() }
    }
}