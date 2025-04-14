pub fn md5() {
    println!("This is a test")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_md5() {
        assert_eq!(1, 1);
    }
}
