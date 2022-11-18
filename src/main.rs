//! Implements the LED-blinker course work on Xilinx PYNQ-Z1 SoC.

// Do not include Rust standard library.
// Rust standard library is not available for bare metal Cortex-A9.
// Thus we use [core](https://doc.rust-lang.org/core/)-library.
#![no_std]
// Open feature gates to certain, currently WIP, features which might become part of Rust in future.
#![feature(start)]

// Define crate's module hierarchy.
mod interrupt;
mod pixel;
mod print;

// `xil_sys` contains the Xilinx Cortex-A9 board support package (BSP) and a Rust FFI.
// We rename the module here as `xil`.
use xil_sys as xil;

// Re-import symbols from pixel without the `pixel::` prefix.
use pixel::*;

// Rust `core` imports for using C-style void-pointers and info for a custom panic implementation.
use core::{ffi::c_void, panic::PanicInfo};

// Declare static globals like in the C-version.
// This is a reasonable way of communicating between threads in interrupt-driven concurrency.
pub static mut A_GLOBAL: usize = 0;

// Define the address of the ordinary LED interface in physical memory.
// Putting bits into the LED address the right way may cause desired blinking of hardware LEDs.
// FIXME: 0x00000000 is not the LED address.
// The correct address can be found in some of the provided documentation.
pub const LED_ADDRESS: *mut u8 = 0x00000000 as *mut u8;

// The #[start] attribute tell's the cross-compiler where to start executing.
// Normally it is not needed.
// Underscore before the argument signals that the parameter is not used.
#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    // Initialize board interrupt functions.
    // N.B. Do not touch this function, concurrency is set up here.
    interrupt::init();

    // An unsafe block for setting up the LED-matrix using the C-API, and for touching a static global.
    unsafe {
        setup_led_matrix();
        // Setting a static global variable requires an `unsafe` block in Rust.
        // Compiler can not verify soundness in a case where an interrupt causes simultaneous access from another thread.
        // Now it is our responsibility to make sure that this does not happen.
        A_GLOBAL = 0;
    }

    unsafe {
        // Enable interrupts.
        // Now control flow can change from main loop to an interrupt handler.
        // This change of control flow happens when an interrupt request (IRQ) is asserted.
        // Direct calls to C API functions (`xil::*`) are `unsafe` by default.
        // The compiler does not verify soundness of C code.
        xil::Xil_ExceptionEnable();
    }

    // Prints up to 64 characters using standard Rust [print formatting](https://doc.rust-lang.org/std/fmt/index.html).
    // You should see this using PuTTY.
    println64!("Hello Rust!");

    // Empty loop to keep the program running while the interrupt handlers do all the work.
    loop {}
}

/// Interrupt handler for switch and buttons.
/// This function gets called on switch and button interrupts.
/// Connected buttons are at bank 2.
///
/// # Arguments
///
/// * `status` - a binding containing one flipped bit to match the source of the
///   interrupt. See line comments of contained `match` statement.
pub unsafe extern "C" fn button_handler(callback_ref: *mut c_void, _bank: u32, status: u32) {
    // Don't mind me, line is for brevity
    // N.B. Removing this line is totally okay
    let _gpio = callback_ref as *mut xil::XGpioPs;

    // TODO: Write code here
    // Tip: use a match-statement to pattern-match button status. The match
    // statement takes the `status` parameter binding and matches it to
    // different binary patterns (eg. 0b001 for decimal 1, or 0b100 for
    // decimal 4). You can use binary, decimal or hex for the match, but I
    // found the binary representation more readable.
    /*
    match status {
        // No buttons are pressed
        0b000000 => {},
        // TODO: match into a pattern here
        // ??? => ???
        // `_` is the 'rest' pattern, that is handled if no other variant matches above
        _ => {},
    }
    */
    // End of your code
}

/// Timer interrupt handler for led matrix update.
/// The function updates only one line (`CHANNEL`) of the matrix per call, but sets `channel` as the next line to be updated.
/// `pub extern "C"` qualifier is required to allow passing the handler to the C API.
pub unsafe extern "C" fn tick_handler(callback_ref: *mut c_void) {
    // Exceptions need to be disabled during screen update.
    xil::Xil_ExceptionDisable();

    // TODO: Write code here

    // End of your code

    // Cast `void*` received from the C API to the "Triple Timer Counter" (TTC)
    // instance pointer. The C API needs to use void pointers to pass data around,
    // because the C specification does not describe abstract data types (ADT).
    let ttc = callback_ref as *mut xil::XTtcPs;

    // Clear timer interrupt status
    // N.B. Do not remove
    let status_event = xil::XTtcPs_GetInterruptStatus(ttc);
    xil::XTtcPs_ClearInterruptStatus(ttc, status_event);

    // Put exceptions back on (previously disabled at the start of the interrupt
    // handler)
    xil::Xil_ExceptionEnable();
}

/// Timer interrupt handler for moving the alien, shooting, and other game
/// logic. See also [tick_handler](fn.tick_handler.html) and its line comments
/// for details.
pub unsafe extern "C" fn tick_handler_1(callback_ref: *mut c_void) {
    // TODO: Write code here

    // End of your code

    // Clear timer interrupt status
    // N.B. Do not remove
    let ttc = callback_ref as *mut xil::XTtcPs;
    let status_event = xil::XTtcPs_GetInterruptStatus(ttc);
    xil::XTtcPs_ClearInterruptStatus(ttc, status_event);
}

/// A custom panic handler for Cortex-A9
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // logs "panicked at '$reason', src/main.rs:27:4" to host stdout
    println64!("{}", info);

    loop {}
}
