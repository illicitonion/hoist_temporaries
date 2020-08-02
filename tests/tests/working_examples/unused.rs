use hoist_temporaries::hoist_temporaries;

#[hoist_temporaries(snack)]
fn main() {
    let snack = "Cheese";
    assert_eq!(snack, "Cheese");
}
