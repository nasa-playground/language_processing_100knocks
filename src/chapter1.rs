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

fn simbolo_elementare(s: &str) -> HashMap<usize, String> {
    let whitelist = vec![1, 5, 6, 7, 8, 9, 15, 19];

    s.split_whitespace()
        .enumerate()
        .fold(HashMap::new(), |mut hash, (i, v)| {
            let v = if whitelist.iter().find(|&&x| x == i).is_some() {
                String::from(&v[0..1])
            } else {
                String::from(&v[0..2])
            };

            hash.insert(i, v);
            hash
        })
}

#[test]
fn reverse_test() {
    assert_eq!(reverse("stressed"), "desserts");
}

#[test]
fn pick_odd_test() {
    assert_eq!(pick_odd("パタトクカシーー"), "パトカー");
}

#[test]
fn add_string_test() {
    assert_eq!(add_string("パトカー", "タクシー"), "パタトクカシーー");
}

#[test]
fn word_len_test() {
    let sentence = "Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics.";

    assert_eq!(
        word_len(sentence),
        vec![3, 1, 4, 1, 6, 9, 2, 7, 5, 3, 5, 8, 9, 7, 10]
    )
}

#[test]
fn simbolo_elementare_test() {
    let s = "Hi He Lied Because Boron Could Not Oxidize Fluorine. New Nations Might Also Sign Peace Security Clause. Arthur King Can.";

    let hash = simbolo_elementare(s);

    // TODO テストケース
    // めんどくさいからいつか書く(書かない)
}
