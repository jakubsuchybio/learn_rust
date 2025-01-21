fn main() {
    let x = if rand::random() { 10 } else { 20 };
    if x == 10 {
        println!("Ten!");
    } else {
        println!("Not ten!");
    }
}
