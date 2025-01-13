use std::error::Error;

pub fn part_one(path: &str) -> Result<usize, Box<dyn Error>> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day_twenty_five_part_one() -> Result<(), Box<dyn Error>> {
        let path = "./assets/day_twenty_five/sample.txt";
        let output = part_one(path)?;
        assert_eq!(1, 1);
        Ok(())
    }
}
