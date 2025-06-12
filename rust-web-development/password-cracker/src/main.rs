use std::env;

use brute::crack;
use db::BuildError;
use wordlist::wordlist_reader;

mod brute;
mod db;
mod md5;

mod wordlist;

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
    } else {
        let respon = crack(args[0].to_uppercase().to_string());
        response.push_str(respon.as_str());
    }
    println!("The output is {}", response);
    Ok(())
}
