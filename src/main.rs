#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![test_runner(sos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use sos::println;

#[panic_handler]
/// Panic handler
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
/// Entry-point of the kernel
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    // Conditional Compilation call to test_main instead of standard main function when run 
    // using `cargo test`
    #[cfg(test)]
    test_main();

    loop {}
}
