#![no_std]
#![deny(warnings)]

pub fn init() {
    oreboot_arch::armv8::init()
}
