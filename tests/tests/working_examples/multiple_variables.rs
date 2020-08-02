use hoist_temporaries::hoist_temporaries;

#[hoist_temporaries]
fn main() {
    #[hoist_temporaries::hoist]
    let mut snack = "Cheese";
    if true {
        snack = &format!("{}burger", snack);
    }
    #[hoist_temporaries::hoist]
    let mut drink = "Lemonade";
    if false {
        drink = &String::from("Orange juice");
    }
    assert_eq!((snack, drink), ("Cheeseburger", "Lemonade"));
}
