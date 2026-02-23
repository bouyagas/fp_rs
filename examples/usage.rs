//! Example: pipe, flow, try_!, opt!, tap.

#[macro_use]
extern crate fp_rs;

fn main() {
    // Pipe: value flows through each step
    let x = pipe!(3, |n: i32| n + 1, |n: i32| n * 2);
    assert_eq!(x, 8);
    println!("pipe: {}", x);

    // Flow: compose functions
    let f = flow!(|n: i32| n + 1, |n: i32| n * 2);
    assert_eq!(f(3), 8);
    println!("flow(3): {}", f(3));

    // Do-notation for Result
    let r: Result<i32, &str> = try_!(
        a <- Ok(2);
        b <- Ok(3);
        return Ok(a + b)
    );
    assert_eq!(r, Ok(5));
    println!("try_: {:?}", r);

    // Do-notation for Option
    let o: Option<i32> = opt!(
        a <- Some(2);
        b <- Some(3);
        return Some(a + b)
    );
    assert_eq!(o, Some(5));
    println!("opt: {:?}", o);

    // Tap (side-effect in pipeline)
    let y = pipe!(42, tap!(|n| println!("tap: {}", n)));
    assert_eq!(y, 42);
}
