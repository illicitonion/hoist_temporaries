use hoist_temporaries::hoist_temporaries;

#[hoist_temporaries]
fn main() {
    #[hoist_temporaries::hoist]
    let snack = "Cheese";
    assert_eq!(snack, "Cheese");
}
