pub fn calculate_price(apples: u32) -> u32 {
    if apples >= 40 {
        apples * 1
    } else {
        apples * 2
    }
}
