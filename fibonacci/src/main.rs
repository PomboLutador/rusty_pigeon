fn main() {
    let n: u8 = 50;
    let result: u64 = fibonacci(n);
    println!("{n}th fibonacci number: {result}");
}

fn fibonacci(n: u8) -> u64 {
    if n == 1 || n == 2 {
        return 1;
    }
    let number_of_loops: u8 = n - 2;
    let mut loop_counter: u8 = 0;
    let mut fib_n_minus_1: u64 = 1;
    let mut fib_n_minus_2: u64 = 1;
    let mut result: u64 = 0;
    while loop_counter < number_of_loops {
        result = fib_n_minus_1 + fib_n_minus_2;
        fib_n_minus_2 = fib_n_minus_1;
        fib_n_minus_1 = result;
        loop_counter = loop_counter + 1;
    }
    return result;
}
