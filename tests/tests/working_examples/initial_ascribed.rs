use hoist_temporaries::hoist_temporaries;

#[hoist_temporaries(snack)]
fn main() {
    let snack: &str;
    {
        snack = &String::from("Cheeseburger");
    }
    assert_eq!(snack, "Cheeseburger");
}
