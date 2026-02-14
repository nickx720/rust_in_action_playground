use std::fs;

use crate::lib::hash::{hash_function_sum, hash_function_sum_variation};
mod hash;

#[derive(Debug)]
pub enum BloomFilter {
    MaybePresent,
    NotPresent,
}

pub struct Bloom {
    hash_count: usize,
    bit_array: Vec<u8>,
    bit_count: usize,
}

// Bit array mental model (packed into bytes):
//
// global bit indexes:
// byte0: 0 1 2 3 4 5 6 7
// byte1: 8 9 10 11 12 13 14 15
// byte2: 16 ...
//
// Mapping a global bit index:
// byte_index = bit_index / 8
// bit_in_byte = bit_index % 8
//
// Mask creation for set/get:
// 1u8      = 0b0000_0001
// 1u8 << 2 = 0b0000_0100
// The mask is OR'ed to set a bit and AND'ed to test a bit.
fn set_bit(bit_array: &mut Vec<u8>, idx: usize) {
    let byte = idx / 8;
    let bit = idx % 8;
    let mask = 1u8 << bit;
    // or -in the mask for updating the position
    bit_array[byte] |= mask;
}

fn get_bit(bit_array: &[u8], idx: usize) -> bool {
    let byte = idx / 8;
    let bit = idx % 8;
    let mask = 1u8 << bit;
    // and to check if it exists
    (bit_array[byte] & mask) != 0
}

impl Bloom {
    pub fn new(number_of_items: usize, false_positive_rate: f64) -> Self {
        let false_positive_rate = false_positive_rate.clamp(0.0f64, 1.0);
        // bit_size_array is the total number of bits in the filter, derived from
        // expected items and target false-positive rate; it drives storage size
        // and is later used to map hashes into valid bit positions.
        // Formula reference: https://en.wikipedia.org/wiki/Bloom_filter#Optimal_number_of_hash_functions
        let bit_count =
            (-(number_of_items as f64 * false_positive_rate.ln()) / (2f64.ln()).powi(2)).ceil();
        let hash_count = ((bit_count / number_of_items as f64) * 2f64.ln()).round() as usize;
        // bit_size_array is a count of bits, but Vec<u8> stores bytes.
        // Round up so we allocate enough bytes to hold all bits.
        let bit_array_length = (bit_count as usize + 7) / 8;
        Self {
            hash_count,
            bit_array: vec![0u8; bit_array_length],
            bit_count: bit_count as usize,
        }
    }
    pub fn insert(&mut self, item: &str) {
        for i in 0..self.hash_count {
            let index = (hash_function_sum(item) + i * hash_function_sum_variation(item))
                % self.bit_count as usize;
            set_bit(&mut self.bit_array, index);
        }
    }
    pub fn exists(&self, item: &str) -> BloomFilter {
        for i in 0..self.hash_count {
            let index = (hash_function_sum(item) + i * hash_function_sum_variation(item))
                % self.bit_count as usize;
            if !get_bit(&self.bit_array, index) {
                return BloomFilter::NotPresent;
            }
        }
        BloomFilter::MaybePresent
    }
}

pub fn make_bloom_with_100() -> Bloom {
    let mut bloom = Bloom::new(100, 0.01);
    let file = fs::read_to_string("./dict.txt").expect("Reading file failed");
    for item in file.split("\n") {
        if item.is_empty() {
            continue;
        }
        bloom.insert(item);
    }
    bloom
}

impl AsRef<[u8]> for Bloom {
    fn as_ref(&self) -> &[u8] {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let bloom = Bloom::new(100, 0.01);
        let item = "test".to_string();
        let response = bloom.exists(&item);
        assert!(matches!(response, BloomFilter::NotPresent));
    }
    #[test]
    fn test_one_item() {
        let mut bloom = make_bloom_with_100();
        let item = "test".to_string();
        bloom.insert(&item);
        let response = bloom.exists(&item);
        assert!(matches!(response, BloomFilter::MaybePresent));
    }
    #[test]
    fn test_one_many() {
        let mut bloom = make_bloom_with_100();
        let item = "test".to_string();
        bloom.insert(&item);
        bloom.insert(&item);
        let response = bloom.exists(&item);
        assert!(matches!(response, BloomFilter::MaybePresent));
    }
}
