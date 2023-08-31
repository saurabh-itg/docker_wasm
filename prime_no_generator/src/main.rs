fn generate_primes_in_range(start: u64, end: u64) -> Vec<u64> {
    let mut primes = Vec::new();
    let mut is_prime = vec![true; (end + 1) as usize];

    for num in 2..=end {
        if is_prime[num as usize] {
            if num >= start {
                primes.push(num);
            }
            let mut multiple = num * num;
            while multiple <= end {
                if multiple >= start {
                    is_prime[multiple as usize] = false;
                }
                // To avoid overflow, we'll use checked_add here
                multiple = match multiple.checked_add(num) {
                    Some(value) => value,
                    None => break,
                };
            }
        }
    }

    primes
}

fn main() {
    let start = 10_000_000;
    let end = 50_000_000;
    let primes = generate_primes_in_range(start, end);

    println!("Prime numbers between {} and {}: {:?}", start, end, primes);
}

