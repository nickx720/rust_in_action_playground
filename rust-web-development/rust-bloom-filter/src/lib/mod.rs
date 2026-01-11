pub fn bloom_filter(number_of_items: usize, false_positive_rate: f64) {
    let false_positive_rate = false_positive_rate.clamp(0.0f64, 1.0);
    let bit_size_array =
        -(number_of_items as f64 * false_positive_rate.ln()) / (2f64.ln()).powi(2).ceil();
    let hash_function = (bit_size_array / number_of_items as f64) * 2f64.ln().round();
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
}
