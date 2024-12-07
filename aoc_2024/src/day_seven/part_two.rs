use std::error::Error;

pub fn part_two(path: &str) -> Result<usize, Box<dyn Error>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_seven_part_two() -> Result<(), Box<dyn Error>> {
        let path = "./assets/day_seven/sample.txt";
        let output = part_two(path)?;
        assert_eq!(output, 41);
        Ok(())
    }
}
