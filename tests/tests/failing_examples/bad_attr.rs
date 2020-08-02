use hoist_temporaries::hoist_temporaries;

#[hoist_temporaries(snack)]
pub fn f() -> (&'static str, i32) {
    let snack = "Cheese";
    let foods = 3_i32;
    (snack, foods)
}

fn main() {}
