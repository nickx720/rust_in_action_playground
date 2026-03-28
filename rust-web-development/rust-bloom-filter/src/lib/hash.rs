// TODO:
// In Bloom filters, k is the number of hash functions. For each item, you produce k hash
// values, turn each into a bit index, and set/check those k bits.
//
// Two common ways:
// - Independent hashes: run k different hash functions on the same item.
// - Double hashing: compute two hashes h1 and h2, then derive the rest with
//   h_i = h1 + i * h2 (mod bit_size). This avoids writing many separate hash
//   functions while still giving k distinct positions.
//
// So "k different hashes for one item" just means you need k different bit
// positions derived from that single item.
//
// Next concrete steps for this file:
// 1) Replace the byte-sum hash with a real string hash that is sensitive to byte order.
//    Hint: FNV-1/FNV-1a are simple enough to implement by hand for learning purposes.
// 2) Make `hash_function_sum_variation` genuinely independent from the first hash.
//    Hint: don't derive it as a trivial linear transform of the same running total.
// 3) Keep the `insert`/`exists` double-hashing pattern, but feed it two stronger base hashes.
// 4) Add tests for collisions you expect to separate.
//    Suggestions: check that `"cat"` and `"tac"` do not hash identically, and compare several
//    short unrelated words to see whether the produced values are better distributed.
// 5) After swapping hashes, rebuild `words.bf` from `dict.txt` so you are not querying an old filter.
pub fn hash_function_sum(item: &str) -> usize {
    let mut total: u32 = 0;
    for indiviual_item in item.bytes() {
        total += indiviual_item as u32;
    }
    total as usize
}
pub fn hash_function_sum_variation(item: &str) -> usize {
    let mut total: u32 = 0;
    for indiviual_item in item.bytes() {
        total += indiviual_item as u32;
    }
    (total * 3 + 1) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_function_sum() {
        let item = "cat".to_string();
        let total = hash_function_sum(&item);
        assert_eq!(total, 312)
    }
    #[test]
    fn test_hash_function_variation() {
        let item = "cat".to_string();
        let total = hash_function_sum_variation(&item);
        assert_eq!(total, 937)
    }
}
