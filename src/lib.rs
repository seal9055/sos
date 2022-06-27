#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

pub mod serial;
pub mod vga_buffer;
pub mod colors;
pub mod testing;
pub mod interrupts;
pub mod gdt;

use crate::testing::Testable;

#[cfg(test)]
use crate::colors::Red;
#[cfg(test)]
use core::panic::PanicInfo;

/// This runner just iterates through all of the tests and executes them
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

#[cfg(test)]
#[no_mangle]
/// Another start function in charge of starting test cases
pub extern "C" fn _start() -> ! {
    init();
    test_main();
    hlt_loop();
}

/// Executes hlt instruction in a loop which stops the cpu until a new interrupt
/// is received
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[cfg(test)]
#[panic_handler]
/// Another panic handler similar to the previous one. In this case however, 
/// information is printed to the serial console so the qemu window does not 
/// have to be launched. This panic handler is called when tests are being run
fn panic(info: &PanicInfo) -> ! {
    serial_println!("{}", Red("[failed]"));
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
/// Enum describing different exit codes used by qemu
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

/// Write a value to port 0xf4 which signals qemu to quit the emulation with an 
/// exit-code
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

/// Handles initialization routines required by the kernel
pub fn init() {
    // Initialize the GDT
    gdt::init();

    // Initialize the IDT
    interrupts::init_idt();

    // Initialize the programmable interrupt controller
    unsafe { interrupts::PICS.lock().initialize() };

    // Enable interrupts on the cpu
    x86_64::instructions::interrupts::enable();
}

