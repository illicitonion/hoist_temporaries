use hoist_temporaries::hoist_temporaries;

#[hoist_temporaries]
pub fn f() {
    let mut snack;
    #[hoist_temporaries::hoist]
    snack = "blah";
    println!("{}", snack);
}

fn main() {}
