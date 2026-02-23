# fp_rs

**Typed functional programming utilities and macros for Rust** — inspired by [fp-ts](https://gcanti.github.io/fp-ts/), [Ramda](https://ramdajs.com/), [Lodash](https://lodash.com/), and [rubico](https://rubico.land/).

## Features

- **`pipe!`** — Pipe a value through a sequence of functions (fp-ts / F# style).
- **`flow!`** — Compose functions into a single function (data-last, Ramda style).
- **`try_!`** — Do-notation for `Result`: bind with `<-`, return with `return`; exits early on `Err`.
- **`opt!`** — Do-notation for `Option`: bind with `<-`, return with `return`; exits early on `None`.
- **`tap!`** — Run a side-effect in a pipeline and return the value.
- **Functions:** `identity`, `constant`, `tap`.

## Usage

Add to `Cargo.toml`:

```toml
[dependencies]
fp_rs = "0.1"
```

Import macros with `#[macro_use]` or call them with the crate prefix (e.g. `fp_rs::pipe!(...)`):

```rust
#[macro_use]
extern crate fp_rs;

use fp_rs::prelude::*;  // identity, constant, tap

// Pipe
let x = pipe!(3, |n: i32| n + 1, |n: i32| n * 2);
assert_eq!(x, 8);

// Flow (compose)
let f = flow!(|n: i32| n + 1, |n: i32| n * 2);
assert_eq!(f(3), 8);

// Do-notation for Result (use parentheses)
let r: Result<i32, &str> = try_!(
    a <- Ok(2);
    b <- Ok(3);
    return Ok(a + b)
);
assert_eq!(r, Ok(5));

// Do-notation for Option
let o: Option<i32> = opt!(
    a <- Some(2);
    b <- Some(3);
    return Some(a + b)
);
assert_eq!(o, Some(5));

// Tap in a pipeline
let y = pipe!(42, tap!(|n| eprintln!("value: {}", n)));
assert_eq!(y, 42);
```

## License

MIT OR Apache-2.0
