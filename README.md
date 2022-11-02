# avr-critical-section

Implementation of `critical-section` for avr targets

Note: this implementation is based on the `avr-device` crate, but is not dependent on it.

## Examples

Use this crate, and then use critical_section as you would normally.

```rust
use critical_section;
use avr_critical_section as _;

fn main() {
    critical_section::with(|cs| {
        // do something with interrupts disabled
    });
}
```
