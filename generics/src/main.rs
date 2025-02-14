/*
 * implement pythagorean theorem, solvign for hypotenuse
 */
use num_traits::ToPrimitive;

fn solve<T: ToPrimitive, U: ToPrimitive>(a: T, b: U) -> f64 {
    // mechanic for allowing to pass in any type of number into solve and have
    // it convert appropriately
    // <T: Float> is an example of a generic type. Like arg list but for types < > is 
    // the types list
    // T isn't special, meant to symbolize type, but could be anything
    // Float is a trait. being used here as a trait bound
    let b_f64 = b.to_f64().unwrap();
    let a_f64 = a.to_f64().unwrap();
    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}

fn main() {
    let a: f32 = 3.0; // if we declare let a: f32 = 3.0;, get an error! s
                 // rust does NOT automatically convert between number types
                 // can use a_f64 = a as f64;
    let b: u8 = 4;

    println!("{}", solve(a, b));
}
