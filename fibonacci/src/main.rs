fn main() {
    let result: u32 = fibonacci(30);

    println!("The Fibonacci result is {result}");
}

fn fibonacci(n: u32) -> u32 {
    let mut current_no: u32 = 1;
    let mut fib_total: u32 = 0;
    let mut previous_no: u32 = 0;

    for _ in 1..n {
        println!("BEFORE fib_total:{fib_total}, current_no:{current_no}, previous:{previous_no}");
        fib_total += current_no + previous_no;
        let temp_no: u32 = current_no;
        current_no = current_no + previous_no;
        previous_no = temp_no;
        println!("AFTER fib_total:{fib_total}, current_no:{current_no}, previous:{previous_no}");
    }

    fib_total
}