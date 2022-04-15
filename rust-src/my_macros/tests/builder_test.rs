use my_macros::Builder;

#[derive(Debug, PartialEq, Builder)]
pub struct Foo {
    bar: String,
}

#[test]
fn builder_test() {
    let foo = Foo {
        bar: String::from("Y"),
    };

    let foo_from_builder: Foo = FooBuilder::new().bar(String::from("Y")).build();

    assert_eq!(foo, foo_from_builder);
}

