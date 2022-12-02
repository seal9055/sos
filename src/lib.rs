#![no_std]
#![feature(abi_x86_interrupt)]

pub mod serial;
pub mod vga_buffer;
pub mod colors;
pub mod interrupts;
pub mod gdt;
pub mod memory;

/// Executes hlt instruction in a loop which stops the cpu until a new interrupt
/// is received
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
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

