use hoist_temporaries::hoist_temporaries;

#[hoist_temporaries]
fn main() {
    let owned = String::from("owned");
    #[hoist_temporaries::hoist]
    let mut food: &str = "Cheese";
    if true {
        food = &owned;
    }
    assert_eq!(food, "owned");
    println!("{}", owned);
}
