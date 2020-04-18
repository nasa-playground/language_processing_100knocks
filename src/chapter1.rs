use std::collections::HashMap;

fn reverse(s: &str) -> String {
    s.chars().rev().collect::<String>()
}

fn pick_odd(s: &str) -> String {
    s.chars()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, v)| v)
        .collect()
}

fn add_string(s1: &str, s2: &str) -> String {
    s1.chars()
        .zip(s2.chars())
        .map(|(c1, c2)| format!("{}{}", c1, c2))
        .collect()
}

fn word_len(s: &str) -> Vec<usize> {
    s.split_whitespace().map(|word| word.len()).collect()
}

#[test]
fn reverse_test() {
    assert_eq!(reverse("stressed"), "desserts");
}

#[test]
fn pick_odd_test() {
    assert_eq!(pick_odd("パタトクカシーー"), "パトカー");
}
