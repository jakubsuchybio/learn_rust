use std::f64;

fn main() {
    let pi = f64::consts::PI;
    let radius = 5.00f64;

    let area = pi * f64::powi(radius, 2);

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    )
}
