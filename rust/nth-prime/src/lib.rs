use std::usize;

pub fn nth(n: u32) -> u32 {
    let mut primes = Vec::new();
    let mut prime_candidate = 0;

    while primes.len() <= n as usize{
        if is_prime(prime_candidate) {
            primes.push(prime_candidate);
        }
        prime_candidate = prime_candidate + 1;
    }

    primes[n as usize]
}

fn is_prime(n: u32) -> bool {
    match n {
        0 => return false,
        1 => return false,
        _ => {
            let mut counter = 2;

            while counter < n {
                if n % counter == 0 {
                    return false;
                }
                counter = counter + 1;
            }
            return true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        assert_eq!(is_prime(0), false);
        assert_eq!(is_prime(1), false);
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(5), true);
    }
}

