use hoist_temporaries::hoist_temporaries;

#[hoist_temporaries]
const x: u8 = 123;

fn main() {}
