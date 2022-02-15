//START IMPORTS
#![no_std]   //Do not link the Rust standard library
#![no_main]  //Disable all Rust-level entry points
use core::panic::PanicInfo;
//END IMPORTS//

//START PANIC HANDLER//
#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
	loop{
	}
}
//END PANIC HANDLER//

//START ENTRYPOINT FUNCTION//
#[no_mangle]  //Do not mangle the name of this specific function
pub extern "C" fn _start() -> ! {  //This function is the entry point for the OS
	//VGA BUFFER DRIVER//
	static HELLO: &[u8] = b"Hello World!";
	let vga_buffer = 0xb8000 as *mut u8;
	for (i, &byte) in HELLO.iter().enumerate() {
		unsafe {
			*vga_buffer.offset(i as isize * 2) = byte;
			*vga_buffer.offset(i as isize * 2 + 1) = 0xb;
		}
	}
	//END VGA BUFFER DRIVER//
	loop{
	}
}
//END ENTRYPOINT FUNCTION//