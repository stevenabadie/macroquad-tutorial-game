use macroquad::prelude::*;

#[macroquad::main("My game")]
async fn main() {
    loop {
        clear_background(YELLOW);
        next_frame().await
    }
}
