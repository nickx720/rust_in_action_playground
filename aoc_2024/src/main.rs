use clap::{Parser, Subcommand};
mod day_eight;
mod day_five;
mod day_four;
mod day_nine;
mod day_one;
mod day_seven;
mod day_six;
mod day_three;
mod day_two;
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    One {
        part_one: Option<bool>,
        part_two: Option<bool>,
    },
    Two {
        part_one: Option<bool>,
        part_two: Option<bool>,
    },
    Three {
        part_one: Option<bool>,
        part_two: Option<bool>,
    },
    Four {
        part_one: Option<bool>,
        part_two: Option<bool>,
    },
    Five {
        part_one: Option<bool>,
        part_two: Option<bool>,
    },
    Six {
        part_one: Option<bool>,
        part_two: Option<bool>,
    },
    Seven {
        part_one: Option<bool>,
        part_two: Option<bool>,
    },
    Eight {
        part_one: Option<bool>,
        part_two: Option<bool>,
    },
    Nine {
        part_one: Option<bool>,
        part_two: Option<bool>,
    },
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::One { part_one, part_two }) => {
            println!("Invoking Day one");
            let file = "./assets/day_one/question.txt";
            match part_one {
                Some(value) => {
                    if *value {
                        let output = day_two::part_one::part_one(file).unwrap();
                        println!("The output of day two first part is {}", output);
                    }
                }
                _ => println!("Ignoring day one part one"),
            }
            match part_two {
                Some(value) => {
                    if *value {
                        let output = day_one::parttwo::part_two(file).unwrap();
                        println!("The output of day one second part is {}", output);
                    }
                }
                _ => println!("Ignoring day one part two"),
            }
        }
        Some(Commands::Two { part_one, part_two }) => {
            println!("Invoking Day Two");
            let file = "./assets/day_two/question.txt";
            match part_one {
                Some(value) => {
                    if *value {
                        let output = day_two::part_one::part_one(file).unwrap();
                        println!("The output of day two first part is {}", output);
                    }
                }
                _ => println!("Ignoring day one part one"),
            }
            match part_two {
                Some(value) => {
                    if *value {
                        let output = day_two::part_two::part_two(file).unwrap();
                        println!("The output of day two second part is {}", output);
                    }
                }
                _ => println!("Ignoring day two part two"),
            }
        }
        Some(Commands::Three { part_one, part_two }) => {
            println!("Invoking Day Three");
            let file = "./assets/day_three/question.txt";
            match part_one {
                Some(value) => {
                    if *value {
                        let output = day_three::part_one::part_one(file).unwrap();
                        println!("The output of day three first part is {}", output);
                    }
                }
                _ => println!("Ignoring day three part one"),
            }
            match part_two {
                Some(value) => {
                    if *value {
                        let output = day_three::part_two::part_two(file).unwrap();
                        println!("The output of day three second part is {}", output);
                    }
                }
                _ => println!("Ignoring day three part two"),
            }
        }
        Some(Commands::Four { part_one, part_two }) => {
            println!("Invoking Day Four");
            let file = "./assets/day_four/question.txt";
            match part_one {
                Some(value) => {
                    if *value {
                        let output = day_four::part_one::part_one(file).unwrap();
                        println!("The output of day four first part is {}", output);
                    }
                }
                _ => println!("Ignoring day four part one"),
            }
            match part_two {
                Some(value) => {
                    if *value {
                        let output = day_four::part_two::part_two(file).unwrap();
                        println!("The output of day four second part is {}", output);
                    }
                }
                _ => println!("Ignoring day four part two"),
            }
        }
        Some(Commands::Five { part_one, part_two }) => {
            println!("Invoking Day Five");
            let file = "./assets/day_five/question.txt";
            match part_one {
                Some(value) => {
                    if *value {
                        let output = day_five::part_one::part_one(file).unwrap();
                        println!("The output of day five first part is {}", output);
                    }
                }
                _ => println!("Ignoring day five part one"),
            }
            match part_two {
                Some(value) => {
                    if *value {
                        let output = day_five::part_two::part_two(file).unwrap();
                        println!("The output of day five second part is {}", output);
                    }
                }
                _ => println!("Ignoring day four part two"),
            }
        }
        Some(Commands::Six { part_one, part_two }) => {
            println!("Invoking Day Six");
            let file = "./assets/day_six/question.txt";
            match part_one {
                Some(value) => {
                    if *value {
                        let output = day_six::part_one::part_one(file).unwrap();
                        println!("The output of day six first part is {}", output);
                    }
                }
                _ => println!("Ignoring day six part one"),
            }
            match part_two {
                Some(value) => {
                    if *value {
                        let output = day_six::part_two::part_two(file).unwrap();
                        println!("The output of day six second part is {}", output);
                    }
                }
                _ => println!("Ignoring day six part two"),
            }
        }
        Some(Commands::Seven { part_one, part_two }) => {
            println!("Invoking Day Seven");
            let file = "./assets/day_seven/question.txt";
            match part_one {
                Some(value) => {
                    if *value {
                        let output = day_seven::part_one::part_one(file).unwrap();
                        println!("The output of day seven first part is {}", output);
                    }
                }
                _ => println!("Ignoring day seven part one"),
            }
            match part_two {
                Some(value) => {
                    if *value {
                        let output = day_seven::part_two::part_two(file).unwrap();
                        println!("The output of day seven second part is {}", output);
                    }
                }
                _ => println!("Ignoring day seven part two"),
            }
        }
        Some(Commands::Eight { part_one, part_two }) => {
            println!("Invoking Day Eight");
            let file = "./assets/day_eight/question.txt";
            match part_one {
                Some(value) => {
                    if *value {
                        let output = day_eight::part_one::part_one(file).unwrap();
                        println!("The output of day eight first part is {}", output);
                    }
                }
                _ => println!("Ignoring day eight part one"),
            }
            match part_two {
                Some(value) => {
                    if *value {
                        let output = day_eight::part_two::part_two(file).unwrap();
                        println!("The output of day eight second part is {}", output);
                    }
                }
                _ => println!("Ignoring day eight part two"),
            }
        }
        Some(Commands::Nine { part_one, part_two }) => {
            println!("Invoking Day Nine");
            let file = "./assets/day_nine/question.txt";
            match part_one {
                Some(value) => {
                    if *value {
                        let output = day_nine::part_one::part_one(file).unwrap();
                        println!("The output of day nine first part is {}", output);
                    }
                }
                _ => println!("Ignoring day nine part one"),
            }
            match part_two {
                Some(value) => {
                    if *value {
                        let output = day_nine::part_two::part_two(file).unwrap();
                        println!("The output of day nine second part is {}", output);
                    }
                }
                _ => println!("Ignoring day nine part two"),
            }
        }
        _ => panic!("Invalid day"),
    }
}
