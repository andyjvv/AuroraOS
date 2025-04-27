#![no_std]
#![no_main]
mod boot;
mod kernel;
use boot::Bootloader;
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut bootloader = Bootloader;
    let kernel = bootloader.load_kernel();
    bootloader.jump_to_kernel(kernel);
    loop {}
}