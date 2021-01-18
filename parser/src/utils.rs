pub fn to_upper_case(input: &str) -> String {
    let mut transformed = String::new();
    let mut uppercase_next = true;
    for ch in input.chars() {
        if uppercase_next {
            uppercase_next = false;
            transformed.push_str(&ch.to_uppercase().to_string());
        } else if ch == '_' {
            uppercase_next = true;
        } else {
            transformed.push(ch);
        }
    }
    transformed
}

pub fn to_snake_case(input: &str) -> String {
    let mut transformed = String::new();
    let mut start = true;
    for ch in input.chars() {
        if ch.is_uppercase() {
            if start {
                start = false;
                transformed.push_str(&ch.to_lowercase().to_string());
            } else {
                transformed.push_str(&format!("_{}", &ch.to_lowercase()));
            }
        } else {
            transformed.push(ch);
        }
    }
    transformed
}
