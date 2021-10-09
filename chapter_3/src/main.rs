use rand::prelude::*;

fn one_in(denominator:u32) -> bool {
    thread_rng().gen_ratio(1, denominator)
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
        }
    }

    fn new_with_data(
        name: &str,
        data: &Vec<u8>
    ) -> File{
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }
    fn read(self: &File, save_to: &mut Vec<u8>) -> usize{
        let mut tmp = self.data.clone();
        let read_length = tmp.len();

        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        read_length
    }
}

fn open(f: File) -> Result<File,String> {
    if one_in(100_000){
        let err_msg = String::from("Permission denied");
        return Err(err_msg);
    }
    Ok(f)
}

fn close(f:File) -> Result<File,String> {
    if one_in(100_000){
        let err_msg = String::from("Interrupted by signal!");
        return Err(err_msg);
    }
    Ok(f)
}



fn main() {
    let f3_data = vec![114,117,115,116,33];
    let mut f2 = File::new_with_data("f2.txt",&f3_data);
    let mut buffer: Vec<u8> = vec![];

    f2 = open(f2).unwrap();
    let f2_length = f2.read(&mut buffer);
    f2 = close(f2).unwrap();

    let text = String::from_utf8_lossy(&buffer);
    println!("{:?}",f2);
    println!("{} is {} bytes long",f2.name,f2_length);
    println!("{}",text);
}
