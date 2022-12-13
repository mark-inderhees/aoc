use std::str::FromStr;

/// Tests if this character is in the string
pub fn char_in_string(c: &char, s: &String) -> bool {
    if s.chars().any(|x| x == *c) {
        return true;
    }
    false
}

/// Gets all values in a string
pub fn get_vals<T>(s: &str) -> Vec<T>
where
    T: FromStr,
{
    let mut values: Vec<T> = vec![];
    let mut line: String = s.clone().to_string();

    // Remove common text seperators
    let remove = [';', ','];
    for r in remove {
        line = line.replace(r, " ");
    }

    // Start adding values to the list
    for item in line.split(" ") {
        let value: Result<T, _> = item.parse();
        if let Ok(v) = value {
            values.push(v);
        }
    }

    values
}

/// Get a single value, the first value found in the line
pub fn get_val<T>(s: &str) -> T
where
    T: FromStr,
    T: Copy,
{
    let values = get_vals(s);
    values[0]
}
