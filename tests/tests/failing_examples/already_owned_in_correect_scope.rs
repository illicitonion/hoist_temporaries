use hoist_temporaries::hoist_temporaries;

#[hoist_temporaries(food)]
fn main() {
    let owned = String::from("owned");
    let mut food: &str = "Cheese";
    if true {
        food = &owned;
    }
    assert_eq!(food, "owned");
    println!("{}", owned);
}
