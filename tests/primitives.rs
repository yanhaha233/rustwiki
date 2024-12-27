use rustwiki;

pub use rustwiki::primitives::should_test;
pub use rustwiki::primitives::should_test::Operator;

#[test]
fn test_primitives() {
    let value = should_test::test_primitives("some_value");
    assert_eq!(value,5);
}

#[test]
fn test_literals_and_operators() {
    let operators = should_test::test_literals_and_operators("+",Operator(5,6,String::from(""),String::from("")));
    assert_eq!(operators,"11");
    let literals = should_test::test_literals_and_operators("AND",Operator(0,0,String::from("True"),String::from("False")));
    assert_eq!(literals,"nothing happened or should use lower-case");
}