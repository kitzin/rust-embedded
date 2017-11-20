//! Prints "Hello, world!" on the OpenOCD console using semihosting
//!
//! ---

#![feature(used)]
#![no_std]

extern crate cortex_m_semihosting;
extern crate stm32f40x;

use core::fmt::Write;
use cortex_m_semihosting::hio;

fn main() {
    let mut stdout = hio::hstdout().unwrap();
    writeln!(stdout, "Hello, world!").unwrap();
}
