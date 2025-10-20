fn main() {
    let nth = 10;
    let mut a = 0;
    let mut b = 1;

    for _ in 0..nth {
        let c = a + b;
        a = b;
        b = c;
    }

    println!("The {}th Fibonacci number is {}", nth, a);

    println!("The {}th Fibonacci number is {}", nth, fibonacci(nth));

    println!("The {}th Fibonacci number is {}", nth, fib_fast(nth as u64));
}

fn fibonacci(n: u32) -> u64 {
    if n == 0 { return 0; }
    let (mut a, mut b) = (0, 1);
    for _ in 1..n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}

fn fib_fast(n: u64) -> u64 {
    fn helper(n: u64) -> (u64, u64) {
        if n == 0 {
            (0, 1)
        } else {
            let (a, b) = helper(n / 2);
            let c = a * (2 * b - a);
            let d = a * a + b * b;
            if n % 2 == 0 {
                (c, d)
            } else {
                (d, c + d)
            }
        }
    }
    helper(n).0
}
