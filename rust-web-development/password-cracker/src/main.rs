use brute::crack;
use md5::md5;

mod brute;
mod md5;

fn main() {
    md5("abc".to_string());
}
