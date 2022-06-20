fn world(hello: &String) {
    println!("{}, World!", hello);
}

fn calculate_length(str: &String) -> usize {
    str.len()
}

fn change(str: &mut String) {
    *str = String::from("Changed");
}

fn main() {
    let s = String::from("Hello");

    // Use this to free the memory allocated to the variable in the heap
    // drop(s);
    
    world(&s);
    
    println!("{}", s);

    // Out of scope
    // {
    //     let mut a = String::from("test");
    // }
    // println!("{}", a);
    
    let s1 = String::from("Hi");
    let s2 = s1.clone();
    
    println!("{} - {}", s1, s2);
    
    let s3 = String::from("Test");
    let s3_len = calculate_length(&s3);
    
    println!("{} - {}", s3, s3_len);
    
    let mut s4 = String::from("To change");
    println!("{}", s4);
    
    change(&mut s4);

    println!("{}", s4);
}


