use std::fs;

fn main() -> Result<(), anyhow::Error> {
    let args = std::env::args().skip(1);
    for file in args {
        let file = fs::canonicalize(file)?;
        let raw_contents = fs::read(file)?;
        for byte in raw_contents.chunks(16) {
            for pair in byte.chunks(2) {
                print!("{:02x}{:02x} ", pair[0], pair[1]);
            }
            println!();
        }
    }
    Ok(())
}
