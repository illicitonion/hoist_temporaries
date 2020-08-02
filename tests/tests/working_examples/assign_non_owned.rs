use hoist_temporaries::hoist_temporaries;

#[hoist_temporaries(food)]
fn main() {
    let mut food = "Beans";
    if true {
        food = &String::from("Toast");
    }
    assert_eq!(food, "Toast");
    food = "Cheese";
    assert_eq!(food, "Cheese");
}
