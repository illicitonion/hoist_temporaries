use hoist_temporaries::hoist_temporaries;

#[hoist_temporaries(snack)]
const x: u8 = 123;

fn main() {}
