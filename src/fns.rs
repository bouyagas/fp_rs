//! Pure function helpers: identity, constant, tap.

/// Returns the argument unchanged. Same as `std::convert::identity` (Rust 1.53+).
/// Useful in pipelines or as a default callback.
///
/// # Example
/// ```
/// use fp_rs::identity;
/// assert_eq!(identity(42), 42);
/// ```
#[inline(always)]
pub fn identity<T>(x: T) -> T {
    x
}

/// Returns a function that always returns `value`, ignoring its argument (Ramda/Lodash `constant`).
///
/// # Example
/// ```
/// use fp_rs::constant;
/// let f = constant(10);
/// assert_eq!(f("ignored"), 10);
/// ```
#[inline(always)]
pub fn constant<T, U>(value: U) -> impl Fn(T) -> U
where
    U: Clone,
{
    move |_: T| value.clone()
}

// constant needs to return a closure that clones. So:
// pub fn constant<T, U>(value: U) -> impl Fn(T) -> U where U: Clone { move _: T => value.clone() }
// But Fn(T) -> U requires the closure to be callable multiple times, so we need to clone each time. Good.

/// Runs `f(value)` for side-effect and returns `value`. Useful in the middle of a pipeline (Ramda/Lodash `tap`).
///
/// # Example
/// ```
/// use fp_rs::tap;
/// let mut side_effect = 0;
/// let out = tap(42, |n| { side_effect = *n; });
/// assert_eq!(out, 42);
/// assert_eq!(side_effect, 42);
/// ```
#[inline(always)]
pub fn tap<T, F>(value: T, f: F) -> T
where
    F: FnOnce(&T),
{
    f(&value);
    value
}
