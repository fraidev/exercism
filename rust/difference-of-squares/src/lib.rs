pub fn square_of_sum(n: u32) -> u32 {
    let mut num = 0;

    for i in 1..=n {
        num = num + i;
    }

    num.pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut num = 0;

    for i in 1..=n {
        num = num + i.pow(2);
    }

    num
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
