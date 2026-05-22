#![allow(clippy::unwrap_used)]

use crate::test::util::new_empty_model;

#[test]
fn representative_range_probability() {
    let mut model = new_empty_model();
    for (row, x, probability) in [
        (1, "1", "0.1"),
        (2, "2", "0.2"),
        (3, "3", "0.3"),
        (4, "4", "0.4"),
    ] {
        model._set(&format!("A{row}"), x);
        model._set(&format!("F{row}"), probability);
    }
    model._set("J1", "=PROB(A1:A4,F1:F4,2,3)");
    model._set("J2", "=PROB(A1:A4,F1:F4,2)");

    model.evaluate();

    assert_eq!(model._get_text("J1"), *"0.5");
    assert_eq!(model._get_text("J2"), *"0.2");
}

#[test]
fn validates_probability_shape_and_sum() {
    let mut model = new_empty_model();
    model._set("A1", "1");
    model._set("A2", "2");
    model._set("F1", "0.4");
    model._set("F2", "0.4");
    model._set("J1", "=PROB(A1:A2,F1:F2,1,2)");
    model._set("J2", "=PROB(A1:A2,F1:F1,1,2)");
    model._set("J3", "=PROB(A1:A2,F1:F2,1,2,3)");

    model.evaluate();

    assert_eq!(model._get_text("J1"), *"#NUM!");
    assert_eq!(model._get_text("J2"), *"#N/A");
    assert_eq!(model._get_text("J3"), *"#ERROR!");
}
