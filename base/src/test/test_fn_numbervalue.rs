#![allow(clippy::unwrap_used)]

use crate::test::util::new_empty_model;

#[test]
fn numbervalue_parses_explicit_separators_and_percent() {
    let mut model = new_empty_model();
    model._set("A1", r#"=NUMBERVALUE("1,234.50",".",",")"#);
    model._set("A2", r#"=NUMBERVALUE("1.234,50",",",".")"#);
    model._set("A3", r#"=NUMBERVALUE("3.5%")"#);

    model.evaluate();

    assert_eq!(model._get_text("A1"), *"1234.5");
    assert_eq!(model._get_text("A2"), *"1234.5");
    assert_eq!(model._get_text("A3"), *"0.035");
}

#[test]
fn numbervalue_rejects_invalid_separator_contract() {
    let mut model = new_empty_model();
    model._set("A1", r#"=NUMBERVALUE("1,234.50",".",".")"#);
    model._set("A2", r#"=NUMBERVALUE("1,234.50","..",",")"#);

    model.evaluate();

    assert_eq!(model._get_text("A1"), *"#VALUE!");
    assert_eq!(model._get_text("A2"), *"#VALUE!");
}
