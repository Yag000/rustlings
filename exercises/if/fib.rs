fn main() {
    let value = 10;
    println!(
        "The rec functiomn gave {} for the value {value}",
        fib_rec(value)
    );
    println!(
        "The it functiomn gave {} for the value {value}",
        fib_it(value)
    );
}

fn fib_rec(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return n;
    }

    return fib_rec(n - 1) + fib_rec(n - 2);
}

fn fib_it(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return n;
    }

    //let mut cur = n;
    let mut values = (0, 1);

    //while cur != 1 {
    for _ in 0..(n - 1) {
        let (b, a) = values;
        values = (a, a + b);
        //cur -= 1;
    }

    return values.1;
}
