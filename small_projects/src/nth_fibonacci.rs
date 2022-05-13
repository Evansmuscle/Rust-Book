pub fn nth_fibonacci(n: i32) -> i32 {
    let mut n1 = 0;
    let mut n2 = 1;
    let mut count = 1;
    let mut fib_number = n1 + n2;

    loop {
        if count == n {
            break;
        }
        
        fib_number = n1 + n2;
        n1 = n2;
        n2 = fib_number;
        
        
        count += 1;
    }
    
    fib_number
}

pub fn nth_fibonacci_formulised(n: i32) -> f64 {
    let golden_mean = (1.0 + 5.0_f64.sqrt()) / 2.0;
    let golden_number = (1.0 - 5.0_f64.sqrt()) / 2.0;
    
    let fib_number = (golden_mean.powi(n) - golden_number.powi(n)) / 5.0_f64.sqrt();
    
    fib_number.round()
}