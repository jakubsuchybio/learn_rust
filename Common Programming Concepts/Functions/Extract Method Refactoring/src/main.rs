fn main() {
    ten_years_ago(1993);
    ten_years_ago(2021);
}

fn ten_years_ago(year: i32) {
    println!("{}: 10 years ago was {}", year, year - 10);
}
