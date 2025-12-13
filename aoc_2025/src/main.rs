use std::env;

use aoc_2025::{
    day5::{day5_partone, day5_parttwo},
    load, read,
};

fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = env::args().collect();
    if let Some(day) = args.get(1) {
        load::load(day)?;
        let path = format!("./assets/day{}/{}.txt", day, day);
        let input = read::read(&path)?;
        let output = day5_partone(&input)?;
        println!("The output for day one is {}", output);
        let output = day5_parttwo(&input)?;
        println!("The output for day two is {}", output);
    } else {
        let path = format!("./sample.txt");
        let input = read::read(&path)?;
        let output = day5_partone(&input)?;
        println!("The output for day one is {}", output);
        let output = day5_parttwo(&input)?;
        println!("The output for day two is {}", output);
    }
    Ok(())
}
