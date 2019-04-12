use your_order;

#[test]
fn returns_expected() {
    assert_eq!(your_order::order("is2 Thi1s T4est 3a"), "Thi1s is2 3a T4est");
    assert_eq!(your_order::order(""), "");
}
