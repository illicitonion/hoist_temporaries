use hoist_temporaries::hoist_temporaries;

#[hoist_temporaries]
fn main() {
    #[hoist_temporaries::hoist]
    let mut snack = "Cheese";
    if true {
        snack = &format!("{}burger", snack);
    }
    assert_eq!(snack, "Cheeseburger");
}
