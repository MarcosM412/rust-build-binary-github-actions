#![no_std]
#![no_main]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[link(name = "c")]
extern "C" {
    fn write(fd: i32, buf: *const i8, count: usize) -> isize;
}

#[no_mangle]
pub extern "C" fn main() -> isize {
    unsafe { write(1, b"Hello, World!\n" as *const u8 as *const i8, 14) };
    0
}
