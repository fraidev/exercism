pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut num: u32 = 0;

    for i in 1..limit {
        for j in factors {
            if *j > 0 && i % j == 0 {
                num = num + i;
                break;
            }
        }
    }

    num
}
