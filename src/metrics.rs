use std::cmp;

/// https://en.wikipedia.org/wiki/Hamming_distance
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

/// https://en.wikipedia.org/wiki/Levenshtein_distance
pub fn levenshtein_distance(str1: &str, str2: &str) -> usize {
    levenshtein_distance_inner(
        &str1.chars().collect::<Vec<_>>(),
        &str2.chars().collect::<Vec<_>>(),
    )
}

// TODO: possible to use memoization?
fn levenshtein_distance_inner(str1: &[char], str2: &[char]) -> usize {
    match (str1.first(), str2.first()) {
        (_, None) => str1.len(),
        (None, _) => str2.len(),
        (c1, c2) if c1 == c2 => levenshtein_distance_inner(&str1[1..], &str2[1..]),
        (_, _) => {
            1 + cmp::min(
                levenshtein_distance_inner(&str1[1..], &str2),
                cmp::min(
                    levenshtein_distance_inner(&str1, &str2[1..]),
                    levenshtein_distance_inner(&str1[1..], &str2[1..]),
                ),
            )
        }
    }
}
