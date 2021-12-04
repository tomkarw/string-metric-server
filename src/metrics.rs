pub fn hamming_distance(str1: &str, str2: &str) -> usize {
    let size_diff = if str1.len() > str2.len() {
        str1.len() - str2.len()
    } else {
        str2.len() - str1.len()
    };
    let match_diff = str1
        .chars()
        .zip(str2.chars())
        .filter(|(c1, c2)| c1 != c2)
        .count();
    size_diff + match_diff
}
