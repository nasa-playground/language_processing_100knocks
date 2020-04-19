use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

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

fn n_gram(s: &str, n: usize) -> Vec<String> {
    (0..(s.len() - n + 1))
        .map(|i| String::from(&s[i..(i + n)]))
        .collect()
}

type H = HashSet<String>;
fn set(s1: &str, s2: &str) -> Vec<H> {
    let x = HashSet::from_iter(n_gram(s1, 2));
    let y = HashSet::from_iter(n_gram(s2, 2));

    vec![&x | &y, &x & &y, &x - &y]
}

fn template(x: &str, y: &str, z: &str) -> String {
    format!("{}時の{}は{}", x, y, z)
}

fn chipher(s: &str) -> String {
    s.chars()
        .map(|c| {
            let code = c as u8;
            if 97 <= code && code <= 122 {
                (219 - code) as char
            } else {
                c
            }
        })
        .collect()
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

    // let hash = simbolo_elementare(s);

    // TODO テストケース
    // めんどくさいからいつか書く(書かない)
}

#[test]
fn n_gram_test() {
    let s = "I am an NLPer";
    let bigram = n_gram(s, 2);

    assert_eq!(
        bigram,
        vec!["I ", " a", "am", "m ", " a", "an", "n ", " N", "NL", "LP", "Pe", "er"]
    )
}

#[test]
fn set_test() {
    let sets = set("paraparaparadise", "paragraph");

    // TODO
    // assert_eq!(vec![HashSet::new(), HashSet::new(), HashSet::new()], sets);
}

#[test]
fn template_test() {
    assert_eq!(template("12", "気温", "22.4"), "12時の気温は22.4")
}

#[test]
fn chipher_test() {
    assert_eq!("zhwu", chipher("asdf"))
}
