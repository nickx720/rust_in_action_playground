use rand::prelude::*;

#[derive(Debug,PartialEq)]
enum FileState {
    Open,
    Closed
}

fn one_in(denominator:u32) -> bool {
    thread_rng().gen_ratio(1, denominator)
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state:FileState::Closed,
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
    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize,String>{
        if self.state != FileState::Open {
            return Err(String::from("File must be open for reading"));
        }
        let mut tmp = self.data.clone();
        let read_length = tmp.len();

        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        Ok(read_length)
    }
}

fn open(mut f: File) -> Result<File,String> {
    f.state = FileState::Open;
   Ok(f)
}

fn close(mut f:File) -> Result<File,String> {
    f.state = FileState::Closed;
    Ok(f)
}



fn main() {
    let mut f2 = File::new("f2.txt");
    let mut buffer: Vec<u8> = vec![];

    if f2.read(&mut buffer).is_err(){
        println!("Error checking is working");
    }

    f2 = open(f2).unwrap();
    let f2_length = f2.read(&mut buffer).unwrap();
    f2 = close(f2).unwrap();

    let text = String::from_utf8_lossy(&buffer);
    println!("{:?}",f2);
    println!("{} is {} bytes long",f2.name,f2_length);
    println!("{}",text);
}
