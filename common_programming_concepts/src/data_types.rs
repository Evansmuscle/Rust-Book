pub fn data_types() -> () {
    let mut x: u8 = 255;
    println!("{}", x);
    // The line below will cause an integer overflow.
    // x = 256;
    let tup: (i32, f64, usize) = (-34, 124.142, 1241153);
    let (a, b, c) = tup;
    println!("{:#?}", tup);
    println!("{}, {}, {}", a, b, c);
    println!("{}, {}, {}", tup.0, tup.1, tup.2);

    let months = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    let mut test: [i32; 3] = [1, 2, 3];

    // Not Possible
    // months = ["a"];
    test = [2, 3, 4];
    let threes = [3; 5];

    // return ();
}
