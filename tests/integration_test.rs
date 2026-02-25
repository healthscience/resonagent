use reson_agent::add;

#[test]
fn test_add_integration() {
    assert_eq!(add(10, 20), 30);
}
