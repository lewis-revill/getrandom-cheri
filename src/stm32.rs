// Copyright 2018 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Implementation for FreeRTOS
use crate::Error;
use core::ffi::{c_void, c_uint};

extern "C" {
    // STM32 HAL Random number generator function
    pub fn HAL_RNG_GenerateRandomNumber(
        phrng: &c_void,
        p_random_number: *mut c_uint,
    );

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
