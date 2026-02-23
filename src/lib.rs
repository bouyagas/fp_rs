//! # fp_rs — Typed functional programming utilities for Rust
//!
//! Macros and functions inspired by [fp-ts](https://gcanti.github.io/fp-ts/),
//! [Ramda](https://ramdajs.com/), [Lodash](https://lodash.com/), and [rubico](https://rubico.land/).
//!
//! ## Macros
//!
//! - [`pipe!`] — Pipe a value through a sequence of functions (fp-ts / F# style).
//! - [`flow!`] — Compose functions into a single function (data-last, Ramda style).
//! - [`try_!`] — Do-notation for `Result`: bind with `<-`, return with `return`; exits early on `Err`.
//! - [`opt!`] — Do-notation for `Option`: bind with `<-`, return with `return`; exits early on `None`.
//! - [`tap!`] — Run a side-effect on a value and return the value (for pipelines).
//!
//! ## Functions
//!
//! - [`identity`] — Returns the argument unchanged.
//! - [`constant`] — Returns a function that always returns the given value.
//! - [`tap`] — Run a side-effect and return the value.
//!
//! ## Example
//!
//! ```rust
//! use fp_rs::prelude::*;
//!
//! // Pipe: value flows through each step
//! let x = fp_rs::pipe!(3, |n: i32| n + 1, |n: i32| n * 2);
//! assert_eq!(x, 8);
//!
//! // Flow: compose functions (same as pipe but produces a function)
//! let f = fp_rs::flow!(|n: i32| n + 1, |n: i32| n * 2);
//! assert_eq!(f(3), 8);
//!
//! // Do-notation for Result
//! let res: Result<i32, &str> = fp_rs::try_!(
//!     a <- Ok(2i32);
//!     b <- Ok(3i32);
//!     return Ok(a + b)
//! );
//! assert_eq!(res, Ok(5));
//!
//! // Do-notation for Option
//! let opt: Option<i32> = fp_rs::opt!(
//!     a <- Some(2);
//!     b <- Some(3);
//!     return Some(a + b)
//! );
//! assert_eq!(opt, Some(5));
//!
//! // Tap for side-effects in a pipeline
//! let y = fp_rs::pipe!(42, fp_rs::tap!(|n| eprintln!("value is {}", n)));
//! assert_eq!(y, 42);
//! ```

pub mod fns;
pub mod macros;
pub mod prelude;

pub use fns::{constant, identity, tap};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipe() {
        let x = pipe!(3, |n: i32| n + 1, |n: i32| n * 2);
        assert_eq!(x, 8);
        let s = pipe!("  hello  ", str::trim, str::to_uppercase);
        assert_eq!(s, "HELLO");
    }

    #[test]
    fn test_flow() {
        let f = flow!(|n: i32| n + 1, |n: i32| n * 2);
        assert_eq!(f(3), 8);
    }

    #[test]
    fn test_try_ok() {
        let r: Result<i32, &str> = try_!(
            a <- Ok(2);
            b <- Ok(3);
            return Ok(a + b)
        );
        assert_eq!(r, Ok(5));
    }

    #[test]
    fn test_try_err() {
        let e: Result<i32, &str> = try_!(
            _unit <- Ok(());
            _err <- Err::<(), &str>("failed");
            return Ok(42)
        );
        assert_eq!(e, Err("failed"));
    }

    #[test]
    fn test_opt_some() {
        let o: Option<i32> = opt!(
            a <- Some(2);
            b <- Some(3);
            return Some(a + b)
        );
        assert_eq!(o, Some(5));
    }

    #[test]
    fn test_opt_none() {
        let o: Option<i32> = opt!(
            _unit <- Some(());
            _none <- None::<()>;
            return Some(42)
        );
        assert_eq!(o, None);
    }

    #[test]
    fn test_tap_macro() {
        let y = pipe!(42, tap!(|_n: &i32| {}));
        assert_eq!(y, 42);
    }

    #[test]
    fn test_identity() {
        assert_eq!(identity(42), 42);
        assert_eq!(identity("hello"), "hello");
    }

    #[test]
    fn test_constant() {
        let f = constant(10);
        assert_eq!(f("ignored"), 10);
        let g = constant("value");
        assert_eq!(g(()), "value");
    }

    #[test]
    fn test_tap_fn() {
        let mut side = 0;
        let out = tap(42, |n| {
            side = *n;
        });
        assert_eq!(out, 42);
        assert_eq!(side, 42);
    }
}
