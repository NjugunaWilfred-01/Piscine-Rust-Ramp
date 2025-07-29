pub fn insert(vec: &mut Vec<String>, val: String) {
    vec.push(val);
    vec.insert()
}

pub fn at_index(slice: &[String], index: usize) -> &str {
    if index < slice.len() {
        &slice[index]
    } else {
        panic!();
    }
}
