use std::collections::vec_deque;

pub fn verse(n: u32) -> String {
    let mut result = String::new();

    let str  = match n {
        0 => String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n"),
        1 => (n.to_string() + " bottle of beer on the wall, " + &n.to_string() + " bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n"),
        2 => (n.to_string() + " bottles of beer on the wall, " + &n.to_string() + " bottles of beer.\nTake one down and pass it around, " + &(n - 1).to_string() + " bottle of beer on the wall.\n"),
        _ => (n.to_string() + " bottles of beer on the wall, " + &n.to_string() + " bottles of beer.\nTake one down and pass it around, " + &(n - 1).to_string() + " bottles of beer on the wall.\n")
    };

    result = result + &str;
    result
}

pub fn sing(start: u32, end: u32) -> String {
    let mut result = String::new();

    let vec = if start < end {
        (start..=end).rev()
    } else {
        (end..=start).rev()
    };

    let last = vec.to_owned().last();

    for i in vec {
        let str = verse(i);

        match last {
            Some(x) => {
                if x == i {
                    result = result + &str
                } else {
                    result = result + &str + "\n"
                }
            }
            None => {}
        }
    }

    result
}
