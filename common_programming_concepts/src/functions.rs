fn test(x: i32) -> i32 {
    x + 1
}

pub fn functions() -> i32 {
    let y = test(10);

    // If you put a semicolon, it's a statement, if you don't put one, it's an expression that can be returned
    y
}
