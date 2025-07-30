pub fn is_empty(v: &str) -> bool {
    v.is_empty()
}

pub fn is_ascii(v: &str) -> bool {
    v.is_ascii()
}

pub fn contains(v: &str, pat: &str) -> bool {
    v.contains(pat)
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    v.split_at(index)
    // v.split(index)
}

pub fn find(v: &str, pat: char) -> usize {
    v.find(pat).unwrap_or(usize::MAX)
    // v.chars().position(|c| c == pat).unwrap_or(0)
}