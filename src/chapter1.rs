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

#[test]
fn reverse_test() {
    assert_eq!(reverse("stressed"), "desserts");
}

#[test]
fn pick_odd_test() {
    assert_eq!(pick_odd("パタトクカシーー"), "パトカー");
}
