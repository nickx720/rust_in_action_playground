use std::env;

use brute::crack;
use wordlist::wordlist_reader;

mod brute;
mod db;
mod md5;

mod wordlist {
    //In this step your goal is to use a word list to speed up the attack. Instead of generated every single possible permutation of letters we’ll use a word list of common passwords. You can get one such list from CrackStation here. Grab the Smaller Wordlist for now.

    use std::error::Error;

    use crate::db::{dbsetup, get_query};

    //Adapt your program to allow the user to specify whether to brute force or use a word list, allowing them to specify the path to the word list. See how quickly you can crack this hash: 2bdb742fc3d075ec6b73ea414f27819a
    pub fn wordlist_reader(hash: &str) -> Result<String, Box<dyn Error>> {
        let _ = dbsetup();
        let _response = get_query(hash)?;
        todo!()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().into_iter().skip(1).collect();
    dbg!(&args);
    if args.len() < 1usize {
        println!("Pass the --wordlist flag with value to check example --wordlist crackme");
        return Ok(());
    }
    if args[0] == "--wordlist".to_owned() {
        let _ = wordlist_reader(&args[1]).unwrap();
    } else {
        crack(args[1].to_uppercase().to_string());
    }
    Ok(())
}
