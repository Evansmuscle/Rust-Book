pub fn fahrenheit_to_celcius(fahrenheit: f64) -> f64 {
    let celsius: f64 = (fahrenheit - 32.0) * (5.0 / 9.0);
    
    celsius
}