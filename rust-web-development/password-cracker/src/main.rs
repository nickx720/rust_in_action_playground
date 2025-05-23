use brute::crack;
use md5::md5;
use wordlist::wordlist_reader;

mod brute;
mod md5;

mod wordlist {
    //In this step your goal is to use a word list to speed up the attack. Instead of generated every single possible permutation of letters we’ll use a word list of common passwords. You can get one such list from CrackStation here. Grab the Smaller Wordlist for now.

    use std::{error::Error, fs::read};

    //Adapt your program to allow the user to specify whether to brute force or use a word list, allowing them to specify the path to the word list. See how quickly you can crack this hash: 2bdb742fc3d075ec6b73ea414f27819a
    pub fn wordlist_reader() -> Result<String, Box<dyn Error>> {
        let file = read("assets/realhuman_phill.txt")?;
        let content: Vec<String> = String::from_utf8_lossy(&file)
            .split("\n")
            .take(10)
            .map(|item| item.to_owned())
            .collect();
        for item in content {
            // TODO confirm if the printed output is valid to compare
            println!("{}", item);
        }
        todo!()
    }
}

fn main() {
    md5("abc".to_string());
    crack("pass".to_uppercase().to_string());
    let _ = wordlist_reader().unwrap();
}
