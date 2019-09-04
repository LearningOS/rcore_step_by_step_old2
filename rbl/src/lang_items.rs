// Rust language items

use core::alloc::Layout;
use core::panic::PanicInfo;

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let location = info.location().unwrap();
    let message = info.message().unwrap();
    println!(
        "\nPANIC in {} at line {} \n\t{}",
        location.file(),
        location.line(),
        message
    );

    loop {}
}

#[lang = "oom"]
fn oom(_: Layout) -> ! {
    panic!("out of memory");
}
