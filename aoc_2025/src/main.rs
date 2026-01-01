use std::{env, time::Instant};

use aoc_2025::{day9::day9_partone, load, read};

fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = env::args().collect();
    if let Some(day) = args.get(1) {
        load::load(day)?;
        let path = format!("./assets/day{}/{}.txt", day, day);
        let input = read::read(&path)?;
        let start = Instant::now();
        let output_one = day9_partone(&input)?;
        //        let output_two = day8_partwo(&input)?;
        let elapsed = start.elapsed();

        println!("The output for day one is {}", output_one);
        //       println!("The output for day two is {}", output_two);
        println!("Runtime: {}µs", elapsed.as_micros());
    } else {
        let path = format!("./sample.txt");
        let input = read::read(&path)?;
        let output = day9_partone(&input)?;
        println!("The output for day one is {}", output);
        //        let output = day8_partwo(&input)?;
        //        println!("The output for day two is {}", output);
    }
    Ok(())
}
