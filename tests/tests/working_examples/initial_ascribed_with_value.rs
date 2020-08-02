use hoist_temporaries::hoist_temporaries;

#[hoist_temporaries(snack)]
fn main() {
    let mut snack: &str = "Cheese";
    if true {
        snack = &format!("{}burger", snack);
    }
    assert_eq!(snack, "Cheeseburger");
}
