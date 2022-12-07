pub fn char_in_string(c: &char, s: &String) -> bool {
    if s.chars().any(|x| x == *c) {
        return true;
    }
    false
}
