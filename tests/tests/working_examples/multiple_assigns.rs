use hoist_temporaries::hoist_temporaries;

#[hoist_temporaries(food)]
fn main() {
    let mut food = "Toast";
    if true {
        food = &format!("{} and peanut butter", food);
    }
    if true {
        food = &format!("{} and jam", food);
    }
    assert_eq!(food, "Toast and peanut butter and jam");
}
