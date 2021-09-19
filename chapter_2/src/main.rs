mod abs;
mod complex;
mod mandelbrot;
use mandelbrot::{calculate_mandelbrot,render_mandelbrot};
fn main() {
    let mandelbrot = calculate_mandelbrot(1000, -2.0, 1.0, -1.0, 1.0, 80, 24);
    render_mandelbrot(mandelbrot);
}
