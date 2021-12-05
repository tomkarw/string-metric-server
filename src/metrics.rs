use std::cmp;

/// https://en.wikipedia.org/wiki/Hamming_distance
pub fn hamming_distance(str1: &str, str2: &str) -> f64 {
    let (size_diff, max_size) = if str1.len() > str2.len() {
        (str1.len() - str2.len(), str1.len())
    } else {
        (str2.len() - str1.len(), str2.len())
    };
    let match_diff = str1
        .chars()
        .zip(str2.chars())
        .filter(|(c1, c2)| c1 != c2)
        .count();
    (size_diff + match_diff) as f64 / max_size as f64
}

/// https://en.wikipedia.org/wiki/Levenshtein_distance
pub fn levenshtein_distance(str1: &str, str2: &str) -> f64 {
    levenshtein_distance_inner(
        &str1.chars().collect::<Vec<_>>(),
        &str2.chars().collect::<Vec<_>>(),
    ) as f64
        / cmp::max(str1.len(), str2.len()) as f64
}

// TODO: possible to use memoization?
fn levenshtein_distance_inner(str1: &[char], str2: &[char]) -> usize {
    match (str1.first(), str2.first()) {
        (_, None) => str1.len(),
        (None, _) => str2.len(),
        (c1, c2) if c1 == c2 => levenshtein_distance_inner(&str1[1..], &str2[1..]),
        (_, _) => {
            1 + cmp::min(
                levenshtein_distance_inner(&str1[1..], str2),
                cmp::min(
                    levenshtein_distance_inner(str1, &str2[1..]),
                    levenshtein_distance_inner(&str1[1..], &str2[1..]),
                ),
            )
        }
    }
}

/// https://en.wikipedia.org/wiki/Jaro%E2%80%93Winkler_distance
/// source: https://www.geeksforgeeks.org/jaro-and-jaro-winkler-similarity/
pub fn jaro_distance(str1: &str, str2: &str) -> f64 {
    if str1 == str2 {
        return 0.0;
    }

    let str1 = str1.chars().collect::<Vec<_>>();
    let str2 = str2.chars().collect::<Vec<_>>();

    let len1 = str1.len();
    let len2 = str2.len();

    let max_dist = (cmp::max(len1, len2) / 2) - 1;

    let mut matched = 0;

    let mut hash1 = vec![false; len1];
    let mut hash2 = vec![false; len2];

    for i in 0..len1 {
        let low = if i > max_dist { i - max_dist } else { 0 };
        let high = cmp::min(len2, i + max_dist + 1);
        for j in low..high {
            if str1[i] == str2[j] && !hash2[j] {
                hash1[i] = true;
                hash2[j] = true;
                matched += 1;
                break;
            }
        }
    }

    if matched == 0 {
        return 1.0;
    }

    let mut transpositions = 0;
    let mut point = 0;

    for i in 0..len1 {
        if hash1[i] {
            while !hash2[point] {
                point += 1;
            }

            if str1[i] != str2[point] {
                transpositions += 1;
            }
            point += 1;
        }
    }

    let transpositions = (transpositions as f64 / 2.0).floor();
    let matched = matched as f64;

    1.0 - ((matched) / len1 as f64 + (matched) / len2 as f64 + (matched - transpositions) / matched)
        as f64
        / 3.0
}
