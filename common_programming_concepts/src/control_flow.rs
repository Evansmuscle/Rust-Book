pub fn control_flows() {
    let x = 10;
    let y = 20;

    if x < y {
        println!("X is less then y!");
    } else {
        println!("X is not less then y!");
    }
    let mut count = 0;
    'counting_up: loop {
        println!("count: {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        
        count += 1;
    }
    
    let mut counter = 0;
    let result = loop {
        counter += 1;
        
        if counter == 10 {
            break counter * 2;
        }
    };
    
    println!("The result is: {}", result);
    
    let mut number = 3;
    while number != 0 {
        println!("{}", number);
        
        number -= 1;
    }
    
    println!("LIFTOFF!!!");
    
    let arr = [10, 20, 30, 40, 50];
    let mut idx = 0;
    
    while idx < arr.len() {
        println!("The value is: {}", arr[idx]);
        
        idx += 1;
    }
    
    for el in arr {
        println!("The element is: {}", el);
    }
    
    for num in (1..=3).rev() {
        println!("{}!", num);
    }
    println!("LIFTOFF!!!!");
}
