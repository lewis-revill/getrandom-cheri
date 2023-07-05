//! Implementation for embedded STM32 devices (possibly with an RTOS)
use crate::Error;
use core::ffi::{c_uint, c_void};

extern "C" {
    // STM32 HAL Random number generator function
    pub fn HAL_RNG_GenerateRandomNumber(phrng: &c_void, p_random_number: *mut c_uint);

    // STM32 HAL RNG handle
    static hrng: c_void;
}

pub fn getrandom_inner(dest: &mut [u8]) -> Result<(), Error> {
    let mut random_number: c_uint = 0;

    for i in 0..dest.len() {
        unsafe {
            HAL_RNG_GenerateRandomNumber(&hrng, &mut random_number);
        }
        dest[i] = (random_number & 0xff) as u8;
    }
    Ok(())
}
