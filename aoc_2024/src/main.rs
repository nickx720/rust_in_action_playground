use clap::{Parser, Subcommand};
mod day_eight;
mod day_eighteen;
mod day_eleven;
mod day_fifteen;
mod day_five;
mod day_four;
mod day_fourteen;
mod day_nine;
mod day_nineteen;
mod day_one;
mod day_seven;
mod day_seventeen;
mod day_six;
mod day_sixteen;
mod day_ten;
mod day_thirteen;
mod day_three;
mod day_twelve;
mod day_twenty;
mod day_twenty_one;
mod day_twenty_two;
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
    Ten {
        part_one: Option<bool>,
        part_two: Option<bool>,
    },
    Eleven {
        part_one: Option<bool>,
        part_two: Option<bool>,
    },
    Twelve {
        part_one: Option<bool>,
        part_two: Option<bool>,
    },
    Thirteen {
        part_one: Option<bool>,
        part_two: Option<bool>,
    },
    Fourteen {
        part_one: Option<bool>,
        part_two: Option<bool>,
    },
    Fifteen {
        part_one: Option<bool>,
        part_two: Option<bool>,
    },
    Sixteen {
        part_one: Option<bool>,
        part_two: Option<bool>,
    },
    Seventeen {
        part_one: Option<bool>,
        part_two: Option<bool>,
    },
    Eighteen {
        part_one: Option<bool>,
        part_two: Option<bool>,
    },
    Nineteen {
        part_one: Option<bool>,
        part_two: Option<bool>,
    },
    Twenty {
        part_one: Option<bool>,
        part_two: Option<bool>,
    },
    TwentyOne {
        part_one: Option<bool>,
        part_two: Option<bool>,
    },
    TwentyTwo {
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
        Some(Commands::Ten { part_one, part_two }) => {
            println!("Invoking Day Ten");
            let file = "./assets/day_ten/question.txt";
            match part_one {
                Some(value) => {
                    if *value {
                        let output = day_ten::part_one::part_one(file).unwrap();
                        println!("The output of day ten first part is {}", output);
                    }
                }
                _ => println!("Ignoring day ten part one"),
            }
            match part_two {
                Some(value) => {
                    if *value {
                        let output = day_ten::part_two::part_two(file).unwrap();
                        println!("The output of day ten second part is {}", output);
                    }
                }
                _ => println!("Ignoring day ten part two"),
            }
        }
        Some(Commands::Eleven { part_one, part_two }) => {
            println!("Invoking Day Eleven");
            let file = "./assets/day_eleven/question.txt";
            match part_one {
                Some(value) => {
                    if *value {
                        let output = day_eleven::part_one::part_one(file).unwrap();
                        println!("The output of day eleven first part is {}", output);
                    }
                }
                _ => println!("Ignoring day eleven part one"),
            }
            match part_two {
                Some(value) => {
                    if *value {
                        let output = day_eleven::part_two::part_two(file).unwrap();
                        println!("The output of day eleven second part is {}", output);
                    }
                }
                _ => println!("Ignoring day eleven part two"),
            }
        }
        Some(Commands::Twelve { part_one, part_two }) => {
            println!("Invoking Day Twelve");
            let file = "./assets/day_twelve/question.txt";
            match part_one {
                Some(value) => {
                    if *value {
                        let output = day_twelve::part_one::part_one(file).unwrap();
                        println!("The output of day twelve first part is {}", output);
                    }
                }
                _ => println!("Ignoring day twelve part one"),
            }
            match part_two {
                Some(value) => {
                    if *value {
                        let output = day_twelve::part_two::part_two(file).unwrap();
                        println!("The output of day twelve second part is {}", output);
                    }
                }
                _ => println!("Ignoring day twelve part two"),
            }
        }
        Some(Commands::Thirteen { part_one, part_two }) => {
            println!("Invoking Day Thirteen");
            let file = "./assets/day_thirteen/question.txt";
            match part_one {
                Some(value) => {
                    if *value {
                        let output = day_thirteen::part_one::part_one(file).unwrap();
                        println!("The output of day thirteen first part is {}", output);
                    }
                }
                _ => println!("Ignoring day thirteen part one"),
            }
            match part_two {
                Some(value) => {
                    if *value {
                        let output = day_thirteen::part_two::part_two(file).unwrap();
                        println!("The output of day thirteen second part is {}", output);
                    }
                }
                _ => println!("Ignoring day thirteen part two"),
            }
        }
        Some(Commands::Fourteen { part_one, part_two }) => {
            println!("Invoking Day Fourteen");
            let file = "./assets/day_fourteen/question.txt";
            match part_one {
                Some(value) => {
                    if *value {
                        let output = day_fourteen::part_one::part_one(file).unwrap();
                        println!("The output of day fourteen first part is {}", output);
                    }
                }
                _ => println!("Ignoring day fourteen part one"),
            }
            match part_two {
                Some(value) => {
                    if *value {
                        let output = day_fourteen::part_two::part_two(file).unwrap();
                        println!("The output of day fourteen second part is {}", output);
                    }
                }
                _ => println!("Ignoring day fourteen part two"),
            }
        }
        Some(Commands::Fifteen { part_one, part_two }) => {
            println!("Invoking Day Fifteen");
            let file = "./assets/day_fifteen/question.txt";
            match part_one {
                Some(value) => {
                    if *value {
                        let output = day_fifteen::part_one::part_one(file).unwrap();
                        println!("The output of day fifteen first part is {}", output);
                    }
                }
                _ => println!("Ignoring day fifteen part one"),
            }
            match part_two {
                Some(value) => {
                    if *value {
                        let output = day_fifteen::part_two::part_two(file).unwrap();
                        println!("The output of day fifteen second part is {}", output);
                    }
                }
                _ => println!("Ignoring day fifteen part two"),
            }
        }
        Some(Commands::Sixteen { part_one, part_two }) => {
            println!("Invoking Day Sixteen");
            let file = "./assets/day_sixteen/question.txt";
            match part_one {
                Some(value) => {
                    if *value {
                        let output = day_sixteen::part_one::part_one(file).unwrap();
                        println!("The output of day sixteen first part is {}", output);
                    }
                }
                _ => println!("Ignoring day sixteen part one"),
            }
            match part_two {
                Some(value) => {
                    if *value {
                        let output = day_sixteen::part_two::part_two(file).unwrap();
                        println!("The output of day sixteen second part is {}", output);
                    }
                }
                _ => println!("Ignoring day sixteen part two"),
            }
        }
        Some(Commands::Seventeen { part_one, part_two }) => {
            println!("Invoking Day Seventeen");
            let file = "./assets/day_seventeen/question.txt";
            match part_one {
                Some(value) => {
                    if *value {
                        let output = day_seventeen::part_one::part_one(file).unwrap();
                        println!("The output of day seventeen first part is {}", output);
                    }
                }
                _ => println!("Ignoring day seventeen part one"),
            }
            match part_two {
                Some(value) => {
                    if *value {
                        let output = day_seventeen::part_two::part_two(file).unwrap();
                        println!("The output of day seventeen second part is {}", output);
                    }
                }
                _ => println!("Ignoring day seventeen part two"),
            }
        }
        Some(Commands::Eighteen { part_one, part_two }) => {
            println!("Invoking Day Eighteen");
            let file = "./assets/day_eighteen/question.txt";
            match part_one {
                Some(value) => {
                    if *value {
                        let output = day_eighteen::part_one::part_one(file).unwrap();
                        println!("The output of day eighteen first part is {}", output);
                    }
                }
                _ => println!("Ignoring day eighteen part one"),
            }
            match part_two {
                Some(value) => {
                    if *value {
                        let output = day_nineteen::part_two::part_two(file).unwrap();
                        println!("The output of day nineteen second part is {}", output);
                    }
                }
                _ => println!("Ignoring day nineteen part two"),
            }
        }
        Some(Commands::Nineteen { part_one, part_two }) => {
            println!("Invoking Day Nineteen");
            let file = "./assets/day_nineteen/question.txt";
            match part_one {
                Some(value) => {
                    if *value {
                        let output = day_nineteen::part_one::part_one(file).unwrap();
                        println!("The output of day nineteen first part is {}", output);
                    }
                }
                _ => println!("Ignoring day nineteen part one"),
            }
            match part_two {
                Some(value) => {
                    if *value {
                        let output = day_nineteen::part_two::part_two(file).unwrap();
                        println!("The output of day nineteen second part is {}", output);
                    }
                }
                _ => println!("Ignoring day nineteen part two"),
            }
        }
        Some(Commands::Twenty { part_one, part_two }) => {
            println!("Invoking Day Twenty");
            let file = "./assets/day_twenty/question.txt";
            match part_one {
                Some(value) => {
                    if *value {
                        let output = day_twenty::part_one::part_one::<100>(file).unwrap();
                        println!("The output of day twenty first part is {}", output);
                    }
                }
                _ => println!("Ignoring day twenty part one"),
            }
            match part_two {
                Some(value) => {
                    if *value {
                        let output = day_twenty::part_two::part_two::<100>(file).unwrap();
                        println!("The output of day twenty second part is {}", output);
                    }
                }
                _ => println!("Ignoring day twenty part two"),
            }
        }
        Some(Commands::TwentyOne { part_one, part_two }) => {
            println!("Invoking Day TwentyOne");
            let file = "./assets/day_twenty_one/question.txt";
            match part_one {
                Some(value) => {
                    if *value {
                        let output = day_twenty_one::part_one::part_one(file).unwrap();
                        println!("The output of day twenty_one first part is {}", output);
                    }
                }
                _ => println!("Ignoring day twenty_one part one"),
            }
            match part_two {
                Some(value) => {
                    if *value {
                        let output = day_twenty_one::part_two::part_two(file).unwrap();
                        println!("The output of day twenty_one second part is {}", output);
                    }
                }
                _ => println!("Ignoring day twenty_one part two"),
            }
        }
        Some(Commands::TwentyTwo { part_one, part_two }) => {
            println!("Invoking Day TwentyTwo");
            let file = "./assets/day_twenty_two/question.txt";
            match part_one {
                Some(value) => {
                    if *value {
                        let output = day_twenty_two::part_one::part_one(file).unwrap();
                        println!("The output of day twenty_two first part is {}", output);
                    }
                }
                _ => println!("Ignoring day twenty_two part one"),
            }
            match part_two {
                Some(value) => {
                    if *value {
                        let output = day_twenty_two::part_two::part_two(file).unwrap();
                        println!("The output of day twenty_two second part is {}", output);
                    }
                }
                _ => println!("Ignoring day twenty_two part two"),
            }
        }
        _ => panic!("Invalid day"),
    }
}
