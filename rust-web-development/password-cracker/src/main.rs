use std::env;

use brute::crack;
use db::BuildError;
use rainbow_table::rainbow_table_lookup;
use wordlist::wordlist_reader;

mod brute;
mod db;
mod md5;

mod wordlist;

mod rainbow_table {
    use std::collections::VecDeque;

    use crate::brute::generate_perumates;
    use crate::md5::md5;

    pub fn rainbow_table_lookup() {
        let mut queue = VecDeque::new();
        let length = 10;
        let permutation = generate_perumates(&mut queue, length);
        dbg!(permutation);
        todo!()
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        fn test_lookup() {}
    }
}

fn main() -> Result<(), BuildError> {
    let args: Vec<String> = env::args().into_iter().skip(1).collect();
    if args.len() < 1usize {
        println!("Pass the --wordlist flag with value to check example --wordlist crackme");
        return Ok(());
    }
    let mut response = String::new();
    if args[0] == "--wordlist".to_owned() {
        if let Ok(respon) = wordlist_reader(&args[1]) {
            response.push_str(respon.as_str());
        } else {
            println!("Not found");
            return Ok(());
        }
    } else if args[0] == "--generate-rainbow-table" {
        rainbow_table_lookup();
    } else {
        let respon = crack(args[0].to_uppercase().to_string());
        response.push_str(respon.as_str());
    }
    println!("The output is {}", response);
    Ok(())
}
