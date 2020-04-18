fn reverse(s: &str) -> String {
    s.chars().rev().collect::<String>()
}

#[test]
fn reverse_test() {
    assert_eq!(reverse("stressed"), "desserts");
}
