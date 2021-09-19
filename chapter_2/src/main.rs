mod abs;
mod complex;
mod mandelbrot;
use mandelbrot::{calculate_mandelbrot,render_mandelbrot};
fn main() {
//    let mandelbrot = calculate_mandelbrot(1000, -2.0, 1.0, -1.0, 1.0, 80, 24);
//    render_mandelbrot(mandelbrot);
let search_term = "picture";
let quote = "\
    Every face, every shop, bedroom, window, public-house
    and dark square is a picture feverishly turned-- in search of
    what ? It is the same with books. What do we seek through millions of pages?";


for (line_num,line) in quote.lines().enumerate(){
    if line.contains(search_term){
        println!("{}: {}",line_num,line);
    }
}
}
