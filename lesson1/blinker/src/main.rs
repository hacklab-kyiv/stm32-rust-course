#![feature(panic_handler)]
#![feature(used)]
#![no_main]
#![no_std]

use core::panic::PanicInfo;

const SYSAHBCLKCTRL: *mut u32 = 0x40048080 as *mut u32;
const GPIO_0_DIR: *mut u32 = 0xA0002000 as *mut u32;
const GPIO_0_NOT: *mut u32 = 0xA0002300 as *mut u32;

//#[inline(always)]
fn mem_write(address: *mut u32, value: u32) {
    unsafe {
        *address = value;
    }
}

//#[inline(always)]
fn mem_read(address: *mut u32) -> u32 {
    unsafe {
        *address
    }
}

extern "C" fn hardfault_handler() {
    loop { }
}

extern "C" fn nmi_handler() {
    loop { }
}

extern "C" fn default_handler() {
    loop { }
}

extern "C" fn reset_handler() {
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

#[link_section = ".reset_handler"]
#[used]
static RESET_HANDLER: [extern "C" fn(); 15] = [
    reset_handler, 
    nmi_handler,
    hardfault_handler,
    default_handler,
    default_handler,
    default_handler,
    default_handler,
    default_handler,
    default_handler,
    default_handler,
    default_handler,
    default_handler,
    default_handler,
    default_handler,
    default_handler,
];

#[panic_handler]
fn panic_impl(info: &PanicInfo) -> ! {
    loop {

    }
}