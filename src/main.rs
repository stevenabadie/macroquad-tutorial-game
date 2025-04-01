use macroquad::prelude::*;

#[macroquad::main("My game")]
async fn main() {
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;
    let speed = 3.0;

    loop {
        clear_background(YELLOW);

        if is_key_down(KeyCode::Right) {
            x += speed;
        }
        if is_key_down(KeyCode::Left) {
            x -= speed;
        }
        if is_key_down(KeyCode::Down) {
            y += speed;
        }
        if is_key_down(KeyCode::Up) {
            y -= speed;
        }

        draw_circle(x, y, 16.0, BLACK);

        next_frame().await
    }
}
