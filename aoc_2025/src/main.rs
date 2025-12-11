use std::env;

use aoc_2025::{
    day4::{day4_partone, day4_parttwo},
    day5::dayfive_partone,
    load, read,
};

fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = env::args().collect();
    if let Some(day) = args.get(1) {
        load::load(day)?;
        let path = format!("./assets/day{}/{}.txt", day, day);
        let input = read::read(&path)?;
        let output = day4_partone(&input)?;
        println!("The output for day one is {}", output);
        let output = day4_parttwo(&input)?;
        println!("The output for day two is {}", output);
    } else {
        let path = format!("./sample.txt");
        let input = read::read(&path)?;
        let output = dayfive_partone(&input)?;
        println!("The output for day one is {}", output);
        //        let output = day4_parttwo(&input)?;
        //        println!("The output for day two is {}", output);
    }
    Ok(())
}
