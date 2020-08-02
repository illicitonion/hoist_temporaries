use hoist_temporaries::hoist_temporaries;

#[hoist_temporaries]
fn main() {
    #[hoist_temporaries::hoist]
    let snack: &str;
    {
        snack = &String::from("Cheeseburger");
    }
    assert_eq!(snack, "Cheeseburger");
}
