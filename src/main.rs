#![no_main]
#![no_std]

use core::panic::PanicInfo;
use sos::println;

use bootloader::{BootInfo, entry_point};

#[panic_handler]
/// Panic handler
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    sos::hlt_loop();
}

entry_point!(kernel_main);

/// Entry-point of the kernel
pub fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!");

    // Initialize the kernel
    sos::init();

    use x86_64::VirtAddr;
    use x86_64::structures::paging::PageTable;
    use sos::memory::active_level_4_table;

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let l4_table = unsafe { active_level_4_table(phys_mem_offset) };

    for (i, entry) in l4_table.iter().enumerate() {
        if !entry.is_unused() {
            println!("L4 Entry {}: {:?}", i, entry);

            let phys = entry.frame().unwrap().start_address();
            let virt = phys.as_u64() + boot_info.physical_memory_offset;
            let ptr = VirtAddr::new(virt).as_mut_ptr();
            let l3_table: &PageTable = unsafe { &*ptr };

            for (i, entry) in l3_table.iter().enumerate() {
                if !entry.is_unused() {
                    println!("L3 Entry {}: {:?}", i, entry);
                }
            }

        }
    }

    println!("No Crash");

    sos::hlt_loop();
}
