//! Functional composition and do-notation macros.

/// Pipes `value` through a sequence of expressions. Each step receives the result of the previous.
/// Inspired by fp-ts `pipe`, F# `|>`, and Elixir `|>`.
///
/// Syntax: `pipe!(value, step1, step2, ...)`. Each step can be a function call, a closure, or a method call (using `.method()`).
///
/// # Example
/// ```
/// use fp_rs::pipe;
///
/// let x = pipe!(3, |n: i32| n + 1, |n: i32| n * 2);
/// assert_eq!(x, 8);
///
/// // With method calls (use .method() and the pipe passes the value as first arg)
/// let s = pipe!("  hello  ", str::trim, str::to_uppercase);
/// assert_eq!(s, "HELLO");
/// ```
#[macro_export]
macro_rules! pipe {
    ($value:expr, $first:expr $(, $rest:expr)*) => {
        $crate::pipe!(@ $value, $first $(, $rest)*)
    };
    ($value:expr,) => { $value };
    ($value:expr) => { $value };

    (@ $acc:expr, $f:expr, $($rest:expr),*) => {
        $crate::pipe!(@ ($f($acc)), $($rest),*)
    };
    (@ $acc:expr, $f:expr) => {
        $f($acc)
    };
}

/// Composes functions into a single function. Data-last: the composed function takes the initial value.
/// Same as Ramda `pipe` / fp-ts `flow`: `flow!(f, g)(x)` ≡ `g(f(x))`.
///
/// # Example
/// ```
/// use fp_rs::flow;
///
/// let f = flow!(|n: i32| n + 1, |n: i32| n * 2);
/// assert_eq!(f(3), 8);
/// ```
#[macro_export]
macro_rules! flow {
    ($first:expr, $($rest:expr),+ $(,)?) => {
        move |x| $crate::pipe!(x, $first, $( $rest ),+)
    };
    ($single:expr $(,)?) => { $single };
}

/// Do-notation for `Result`. Bind with `x <- expr;`, return with `return expr`. Exits early on `Err`.
///
/// # Example
/// ```
/// let r: Result<i32, &str> = fp_rs::try_!(
///     a <- Ok(2);
///     b <- Ok(3);
///     return Ok(a + b)
/// );
/// assert_eq!(r, Ok(5));
///
/// let e: Result<i32, &str> = fp_rs::try_!(
///     _ok <- Ok(());
///     _err <- Err::<i32, &str>("failed");
///     return Ok(42)
/// );
/// assert_eq!(e, Err("failed"));
/// ```
#[macro_export]
macro_rules! try_ {
    ( $($body:tt)* ) => {
        $crate::__try_impl!($($body)*)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __try_impl {
    ($var:ident <- $expr:expr; $($rest:tt)*) => {
        match $expr {
            Ok($var) => $crate::__try_impl!($($rest)*),
            Err(e) => Err(e),
        }
    };
    (return $e:expr) => { $e };
    (return $e:expr;) => { $e };
    ($e:expr) => { Ok($e) };
    ($e:expr;) => { Ok($e) };
}

// Now opt! for Option.
#[macro_export]
macro_rules! opt {
    ( $($body:tt)* ) => {
        $crate::__opt_impl!($($body)*)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __opt_impl {
    ($var:ident <- $expr:expr; $($rest:tt)*) => {
        match $expr {
            Some($var) => $crate::__opt_impl!($($rest)*),
            None => None,
        }
    };
    (return $e:expr) => { $e };
    (return $e:expr;) => { $e };
    ($e:expr) => { Some($e) };
    ($e:expr;) => { Some($e) };
}

/// Runs a side-effect on the piped value and returns the value. Use in `pipe!` for logging or debugging.
///
/// # Example
/// ```
/// use fp_rs::pipe;
/// use fp_rs::tap;
///
/// let x = pipe!(42, tap!(|n| eprintln!("value: {}", n)));
/// assert_eq!(x, 42);
/// ```
#[macro_export]
macro_rules! tap {
    ($f:expr) => {
        move |value| $crate::tap(value, $f)
    };
}
