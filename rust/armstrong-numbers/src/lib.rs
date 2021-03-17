pub fn is_armstrong_number(num: u32) -> bool {
    let num_as_string = num.to_string();

    let armstrong = num_as_string.chars().fold(0, |acc, i| {
        acc + (i.to_digit(10).unwrap().pow(num_as_string.len() as u32))
    });
    
    armstrong == num
}
