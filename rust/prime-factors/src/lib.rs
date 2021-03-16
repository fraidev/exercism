use std::u64;

pub fn factors(n: u64) -> Vec<u64> {
    let mut vec = Vec::new();
    let mut number = n;

    loop {
        let lower_prime = lower_prime(number);

        match lower_prime {
            Some(x) => {
                vec.push(x);
                number = number / x;
            }
            None => {
                if number != 1 {
                    vec.push(number);
                }
                break;
            }
        }
    }

    vec
}

fn lower_prime(n: u64) -> Option<u64> {
    for i in 2..n {
        if n % i == 0 {
            return Some(i);
        }
    }

    None
}
