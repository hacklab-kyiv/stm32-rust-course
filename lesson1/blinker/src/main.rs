#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly
// extern crate panic_itm; // logs messages over ITM; requires ITM support
// extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;


const SYSAHBCLKCTRL: *mut u32 = 0x40048080 as *mut u32;
const GPIO_0_DIR: *mut u32 = 0xA0002000 as *mut u32;
const GPIO_0_NOT: *mut u32 = 0xA0002300 as *mut u32;

#[inline(always)]
fn mem_write(address: *mut u32, value: u32) {
    unsafe {
        *address = value;
    }
}

#[inline(always)]
fn mem_read(address: *mut u32) -> u32 {
    unsafe {
        *address
    }
}

#[entry]
fn main() -> ! {
    let mut reg = mem_read(SYSAHBCLKCTRL);
    reg |= 1 << 6;
    mem_write(SYSAHBCLKCTRL, reg);
    mem_write(GPIO_0_DIR, 1 << 4);

    loop {
        //for i in 0..1000 {

        //}
        mem_write(GPIO_0_NOT, 1 << 4);
    }
}
