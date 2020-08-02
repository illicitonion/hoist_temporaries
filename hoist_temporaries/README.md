# hoist_temporaries

Procedural macro to extend the lifetimes of temporary variables.

[![crates.io](https://img.shields.io/crates/v/hoist_temporaries.svg)](https://crates.io/crates/hoist_temporaries)
[![Documentation](https://docs.rs/hoist_temporaries/badge.svg)](https://docs.rs/hoist_temporaries)
[![Build Status](https://travis-ci.org/illicitonion/hoist_temporaries.svg?branch=main)](https://travis-ci.org/illicitonion/hoist_temporaries)

## Examples

See [working examples](https://github.com/illicitonion/hoist_temporaries/blob/main/tests/working_examples) for more examples.

```rust
#[hoist_temporaries::hoist_temporaries]
fn main() {
    #[hoist_temporaries::hoist]
    let mut snack = "Cheese";
    if true {
        // Without hoist_temporaries, this would error because the format!() returns a temporary which would otherwise be dropped because it has no owner.
        snack = &format!("{}burger", snack);
    }
    assert_eq!(snack, "Cheeseburger");
}
```
