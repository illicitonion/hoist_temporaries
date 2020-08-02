use hoist_temporaries::hoist_temporaries;

#[hoist_temporaries]
fn main() {
    #[hoist_temporaries::hoist]
    let mut food = "Beans";
    if true {
        food = &String::from("Toast");
    }
    assert_eq!(food, "Toast");
    food = "Cheese";
    assert_eq!(food, "Cheese");
}
