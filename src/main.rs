#![no_std]
#![no_main]  
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"] 
#![feature(abi_x86_interrupt)]
mod vga_buffer;
mod interrupts;
use bootloader::BootInfo;
use bootloader::bootinfo::MemoryRegionType;
// Rebuild with: cargo bootimage
// Run with:
// qemu-system-x86_64 -display gtk -drive format=raw,file=target/x86_64-hagoll_os/debug/bootimage-hagoll_os.bin
//Run test with: 
// cargo test --target x86_64-hagoll_os.json
//Run exclusively in terminal with:
//qemu-system-x86_64 -accel tcg -display curses -drive format=raw,file=target/x86_64-hagoll_os/debug/bootimage-hagoll_os.bin
//If run failed, clean project and rebuild with:
//  cargo clean

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    println!("Starting HagollOS...");

    interrupts::init_idt();

    println!("Physical memory map:");
    let mut usable_bytes = 0u64;

    for region in boot_info.memory_map.iter() {
        let start = region.range.start_addr();
        let end = region.range.end_addr();

        println!(
            "{:#x}..{:#x}: {:?}",
            start,
            end,
            region.region_type
        );

        if region.region_type == MemoryRegionType::Usable {
            usable_bytes += end - start;
        }
    }

    println!("Usable RAM: {} KiB", usable_bytes / 1024);

    #[cfg(test)]
    test_main();

    loop {
        x86_64::instructions::hlt();
    }
}

#[panic_handler] //handles panic. (duh)
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)] //this is used to run tests only when the test configuration is enabled.
pub fn test_runner(tests: &[&dyn Fn()]) { //dyn means that the type of the function is not known at compile time, and it will be determined at runtime.
    println!("Running {} tests", tests.len());

    for test in tests {
        test(); //run the test function.
    }
}

#[test_case] //test_case is a custom attribute that marks a function as a test case. It is used in conjunction with the test_runner function to run tests.
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}
