use markdownlib::{runfromlib, server};
fn main() {
    //    runfromlib("./docs").unwrap();
    server::server().unwrap();
}
