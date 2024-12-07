use std::error::Error;

pub fn part_one(path: &str) -> Result<usize, Box<dyn Error>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_seven_part_one() -> Result<(), Box<dyn Error>> {
        let path = "./assets/day_seven/sample.txt";
        let output = part_one(path)?;
        assert_eq!(output, 41);
        Ok(())
    }
}
