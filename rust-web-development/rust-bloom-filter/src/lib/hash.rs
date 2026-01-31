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
pub fn hash_function_sum(item: &str) -> u32 {
    let mut total: u32 = 0;
    for indiviual_item in item.bytes() {
        total += indiviual_item as u32;
    }
    total
}
pub fn hash_function_sum_variation(item: &str) -> u32 {
    let mut total: u32 = 0;
    for indiviual_item in item.bytes() {
        total += indiviual_item as u32;
    }
    total * 3 + 1
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
