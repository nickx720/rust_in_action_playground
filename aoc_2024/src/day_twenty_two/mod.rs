pub mod part_one;
pub mod part_two;

fn seq(secret: i64, length: usize) -> (Vec<i64>, Vec<i64>) {
    let mut prices = vec![];
    let mut changes = vec![];
    let mut current = secret;
    for _ in 1..=length {
        let previous = current;
        next(&mut current);
        prices.push(current % 10);
        changes.push((current % 10) - (previous % 10));
    }
    (prices, changes)
}

fn next(secret: &mut i64) -> i64 {
    mix(secret, 64 * *secret);
    prune(secret);
    mix(secret, *secret / 32);
    prune(secret);
    mix(secret, *secret * 2048);
    prune(secret);
    *secret
}

fn mix(secret: &mut i64, x: i64) {
    *secret ^= x
}

fn prune(secret: &mut i64) {
    *secret %= 16777216
}
