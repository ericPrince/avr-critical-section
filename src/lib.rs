//! Implementation of `critical-section` for avr targets
//!
//! Note: this implementation is based on the `avr-device` crate, but is not dependent on it.
//!
//! ## Examples
//!
//! Use this crate, and then use critical_section as you would normally.
//!
//! ```ignore
//! use critical_section;
//! use avr_critical_section as _;
//!
//! fn main() {
//!     critical_section::with(|cs| {
//!         // do something with interrupts disabled
//!     });
//! }
//! ```

#![no_std]
// #![cfg(target_arch = "avr")]
#![feature(asm_experimental_arch)]

use critical_section::{set_impl, Impl, RawRestoreState};

use core::arch::asm;

struct SingleCoreCriticalSection;
set_impl!(SingleCoreCriticalSection);

unsafe impl Impl for SingleCoreCriticalSection {
    unsafe fn acquire() -> RawRestoreState {
        // return whether or not interrupts were disabled
        let sreg_restore;
        // Store current state
        unsafe {
            asm!(
                "in {sreg}, 0x3F",
                sreg = out(reg) sreg_restore,
            )
        };
        sreg_restore
    }

    unsafe fn release(sreg_restore: RawRestoreState) {
        asm!(
            "out 0x3F, {sreg}",
            sreg = in(reg) sreg_restore,
        );
    }
}
