use brute::crack;
use md5::md5;

mod md5;

mod brute {
    //    In this step your goal is to crack an MD5 password by brute force. To do that you’ll want to generate all the possible permutations of valid password characters up to a predefined length, then hash them and compare to a pre-determined hashed password.
    //
    //As a test case try some four letter passwords and brute force them. Here’s a couple you could try:
    //
    //7a95bf926a0333f57705aeac07a362a2
    //08054846bbc9933fd0395f8be516a9f9
    //
    //This is the equivalent of incremental mode in John the Ripper.
    pub fn crack(item: String) {
        todo!()
    }
}
fn main() {
    md5("abc".to_string());
    crack("abc".to_string());
}
