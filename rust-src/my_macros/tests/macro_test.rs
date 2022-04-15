use my_macros::EnumString;

#[derive(EnumString)]
enum E {
    Foo,
    Bar(String),
}

#[test]
fn test_foo() {
    assert!(matches!(E::from_str("foo").unwrap(), E::Foo))
}

#[test]
fn test_param_member() {
    assert!(matches!(E::from_str("bar").unwrap(), E::Bar(_)))
}

#[test]
fn bad_convert() {
    assert!(E::from_str("otherShit").is_err());
}
