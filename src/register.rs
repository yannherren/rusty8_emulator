use num_traits::{CheckedAdd, Unsigned, WrappingAdd, one, zero, CheckedShr};
use std::ops::{BitAnd, BitOr, BitXor};

pub trait UnsignedNum:
    Copy
    + PartialOrd
    + Unsigned
    + BitOr<Output = Self>
    + BitXor<Output = Self>
    + BitAnd<Output = Self>
    + CheckedAdd<Output = Self>
    + CheckedShr<Output = Self>
    + WrappingAdd<Output = Self>
{
}

impl UnsignedNum for u8 {}
impl UnsignedNum for u16 {}
impl UnsignedNum for u32 {}
impl UnsignedNum for u64 {}
impl UnsignedNum for u128 {}

pub struct Register<T: UnsignedNum> {
    pub value: T,
}

impl<T: UnsignedNum> Register<T> {
    pub fn new() -> Self {
        Register { value: T::zero() }
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

    pub fn shr(&mut self, value: T, shift_count: T) -> T {
        let mut vf_flag = T::zero();
        if value & T::one() == T::one() {
            vf_flag = T::one();
        }
        self.value = self.value >> shift_count;
        vf_flag
    }

    pub fn add_with_carry(&mut self, value: T) -> T {
        let carry = self.value.checked_add(&value).is_none();
        self.value = self.value.wrapping_add(&value);
        if carry { T::zero() } else { T::one() }
    }

    pub fn sub_with_borrow(&mut self, value: T) -> T {
        let mut borrow = T::zero();
        if value > self.value {
            borrow = T::one();
        }
        self.set(self.value - value);

        borrow
    }
}