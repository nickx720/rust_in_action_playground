// Reference: https://en.wikipedia.org/wiki/Fowler%E2%80%93Noll%E2%80%93Vo_hash_function
#[allow(dead_code)]
pub fn hash_function_sum(item: &str) -> usize {
    let mut total: u32 = 0;
    for indiviual_item in item.bytes() {
        total += indiviual_item as u32;
    }
    total as usize
}
#[allow(dead_code)]
pub fn hash_function_sum_variation(item: &str) -> usize {
    let mut total: u32 = 0;
    for indiviual_item in item.bytes() {
        total += indiviual_item as u32;
    }
    (total * 3 + 1) as usize
}
const FNV_PRIME: u64 = 0x100000001b3;
const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
pub fn hash_function_fnv_1(item: &str) -> usize {
    let mut hash = FNV_OFFSET_BASIS;
    for byte in item.bytes() {
        hash = hash.wrapping_mul(FNV_PRIME);
        hash ^= byte as u64;
    }
    hash as usize
}
pub fn hash_function_fnv_1a(item: &str) -> usize {
    let mut hash = FNV_OFFSET_BASIS;
    for byte in item.bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash as usize
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
    #[test]
    fn test_hash_function_fnv_1() {
        let item = "cat".to_string();
        let total = hash_function_fnv_1(&item);
        assert_eq!(total, 15624606792861450203)
    }
    #[test]
    fn test_hash_function_fnv_1_unique() {
        let item = "cat".to_string();
        let item_two = "tac".to_string();
        let cat = hash_function_fnv_1(&item);
        let tac = hash_function_fnv_1(&item_two);
        assert_ne!(cat, tac)
    }
}
