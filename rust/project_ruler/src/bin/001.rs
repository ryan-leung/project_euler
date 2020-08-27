fn belowten(n: i64) -> i64{
    let mut result : i64 = 0;
    for x in 0..n {
        if x % 3 == 0 {
            result += x
        } else if x % 5 == 0 {
            result += x
        }
    }
    return result
} 


fn main() {
    // If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
    // Find the sum of all the multiples of 3 or 5 below 1000.
    let result = belowten(1000);
    println!("{}", result)
}
