use std::fs;

#[derive(Debug)]
pub enum BloomFilter {
    MaybePresent,
    NotPresent,
}

pub struct Bloom {
    number_of_items: usize,
    false_positive_rate: f64,
}

impl Bloom {
    pub fn new(number_of_items: usize, false_positive_rate: f64) -> Self {
        let false_positive_rate = false_positive_rate.clamp(0.0f64, 1.0);
        let bit_size_array =
            -(number_of_items as f64 * false_positive_rate.ln()) / (2f64.ln()).powi(2).ceil();
        let hash_function = (bit_size_array / number_of_items as f64) * 2f64.ln().round();
        todo!()
    }
    pub fn insert(&self, _item: &str) -> Self {
        todo!()
    }
    pub fn exists(&self, item: &str) -> BloomFilter {
        todo!()
    }
}

pub fn make_bloom_with_100() -> Bloom {
    let bloom = Bloom::new(100, 0.01);
    let file = fs::read_to_string("./dict.txt").expect("Reading file failed");
    for item in file.split("\n") {
        if item.is_empty() {
            continue;
        }
        bloom.insert(item);
    }
    bloom
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let bloom = make_bloom_with_100();
        let item = "test".to_string();
        let response = bloom.exists(&item);
        assert!(matches!(response, BloomFilter::NotPresent));
    }
    #[test]
    fn test_one_item() {
        let bloom = make_bloom_with_100();
        let item = "test".to_string();
        bloom.insert(&item);
        let response = bloom.exists(&item);
        assert!(matches!(response, BloomFilter::MaybePresent));
    }
    #[test]
    fn test_one_many() {
        let bloom = make_bloom_with_100();
        let item = "test".to_string();
        bloom.insert(&item);
        bloom.insert(&item);
        let response = bloom.exists(&item);
        assert!(matches!(response, BloomFilter::MaybePresent));
    }
}
