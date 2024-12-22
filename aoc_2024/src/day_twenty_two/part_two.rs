use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

use crate::day_twenty_two::seq;

pub fn part_two(path: &str) -> Result<i64, Box<dyn Error>> {
    let file = File::open(path)?;
    let input = BufReader::new(file)
        .lines()
        .map_while(Result::ok)
        .map(|item| item.parse::<i64>().expect("Number is not valid"))
        .collect::<Vec<i64>>();
    let mut sales = HashMap::new();

    input.into_iter().for_each(|secret| {
        let (prices, changes) = seq(secret, 2000);
        let mut bananas = HashMap::new();
        let n = prices.len();
        for i in 3..n {
            let sequence = &changes[i - 3..=i];
            let price = prices[i];
            bananas.entry(sequence).or_insert(price);
        }
        for (k, v) in bananas {
            let key = k.iter().join(",");
            //sales.insert(key, v);
            *sales.entry(key).or_insert(0) += v;
        }
    });
    Ok(*sales
        .iter()
        .max_by(|a, b| a.1.cmp(b.1))
        .expect("Failed comparison")
        .1)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn day_twenty_two_part_two() -> Result<(), Box<dyn Error>> {
        let path = "./assets/day_twenty_two/sample2.txt";
        let output = part_two(path)?;
        assert_eq!(output, 23);
        Ok(())
    }
}
