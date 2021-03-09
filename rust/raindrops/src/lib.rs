pub fn raindrops(n: u32) -> String {
    let mut has_factor = false;
    let mut result = String::new();

    if n % 3 == 0 {
        result.push_str("Pling");
        has_factor = true;
    }
    if n % 5 == 0 {
        result.push_str("Plang");
        has_factor = true;
    }
    if n % 7 == 0 {
        result.push_str("Plong");
        has_factor = true;
    }

    if has_factor {
        return result;
    }

    n.to_string()
}
