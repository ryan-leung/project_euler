fn prime_seive(n: u64) {
    let mut buffer: [i32; n] = [0; n];
}

fn is_prime_naive(numb: &i64) -> bool {
    let mut i = i64::from(3);
    while &i < numb {
        if numb % &i == 0 {
            return false
        }
        i = i + 2;
    }
    return true;
}

fn main() {
    let result = is_prime_naive(&131);
    println!("{}", result)
}