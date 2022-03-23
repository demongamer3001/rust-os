
#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // println!("\nKernel Panic\n");
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // println!("Hello, World!\n");

    loop {}
}

fn print(String text) -> ! {
    // 0xB0000
    
}
