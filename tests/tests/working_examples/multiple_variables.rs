use hoist_temporaries::hoist_temporaries;

#[hoist_temporaries(drink, snack)]
fn main() {
    let mut snack = "Cheese";
    if true {
        snack = &format!("{}burger", snack);
    }
    let mut drink = "Lemonade";
    if false {
        drink = &String::from("Orange juice");
    }
    assert_eq!((snack, drink), ("Cheeseburger", "Lemonade"));
}
