use std::env;

use brute::crack;
use db::BuildError;
use wordlist::wordlist_reader;

mod brute;
mod db;
mod md5;

mod wordlist {

    use std::error::Error;

    use crate::db::{dbsetup, get_query};

    //Adapt your program to allow the user to specify whether to brute force or use a word list, allowing them to specify the path to the word list. See how quickly you can crack this hash: 2bdb742fc3d075ec6b73ea414f27819a
    pub fn wordlist_reader(hash: &str) -> Result<String, Box<dyn Error>> {
        let _ = dbsetup();
        let response = get_query(hash.trim())?;
        Ok(response.original)
    }
}

fn main() -> Result<(), BuildError> {
    let args: Vec<String> = env::args().into_iter().skip(1).collect();
    dbg!(&args);
    if args.len() < 1usize {
        println!("Pass the --wordlist flag with value to check example --wordlist crackme");
        return Ok(());
    }
    let mut response = String::new();
    if args[0] == "--wordlist".to_owned() {
        // TODO Use if let to downcast error
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
