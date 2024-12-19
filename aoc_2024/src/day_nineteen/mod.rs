use std::collections::HashSet;

pub mod part_one;
pub mod part_two;

fn count_ways(towel_patterns: &HashSet<String>, design: &str) -> usize {
    let length = design.len();
    let mut dp = vec![0; length + 1];
    dp[0] = 1;
    for item in 1..=length {
        for pattern in towel_patterns {
            let pattern_length = pattern.len();
            if item >= pattern_length && &design[item - pattern_length..item] == pattern {
                dp[item] += dp[item - pattern_length];
            }
        }
    }
    dp[length]
}
