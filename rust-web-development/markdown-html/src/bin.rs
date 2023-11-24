use markdownlib::runfromlib;
fn main() {
    runfromlib("./docs").unwrap();
    //markdownlib::server::server().unwrap();
}
