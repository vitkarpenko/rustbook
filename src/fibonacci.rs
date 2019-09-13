pub fn fib(n: i32) -> i32 {
    if n <= 0 {
        panic!("Incorrect n.")
    }
    let mut fib = (1, 1);
    for _ in 1..n {
        fib = (fib.1, fib.0 + fib.1);
    }
    fib.0
}

pub fn fib_loop_break_temp_swap(mut n: i32) -> i32 {
    if n <= 0 { panic!("Incorrect n.") }
    let (mut a, mut b) = (0, 1);
    loop {
        let temp = a;
        a = b;
        b = b + temp;
        n -= 1;
        if n == 0 {
            break a;
        }
    }
}

pub fn fib_match(n: i32) -> i32 {
    if n <= 0 {
        panic!("Incorrect n.")
    }
    match n {
        1 => 1,
        2 => 1,
        _ => fib_match(n - 1) + fib_match(n - 2)
    }
}