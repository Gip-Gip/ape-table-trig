# Ape Table Trigonometry
## *Implementations of sin, cos, and tan using precalculated tables.*

Using these functions can significantly improve performance on systems with
limited to no naitive floating point support, like the RP2040. Designed to
be no_std compatible out of the box.

# Example:

```rust
use ape_table_trig::*;

// Table has an accuracy down to 1πmrad
static TABLE: [f32; 1000] = trig_table_gen_f32!(1000);

fn main() {
    let table = TrigTableF32::new(&TABLE);

    // Calculate the sine of 1π radians
    let sine = table.sin(PI_F32);
}
```
