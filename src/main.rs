use macroquad::{miniquad::window::screen_size, prelude::*, rand::ChooseRandom};

struct Square {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
    color: Color,
}

impl Square {
    fn rect(&self) -> Rect {
        Rect {
            x: self.x - self.size / 2.0,
            y: self.y - self.size / 2.0,
            w: self.size,
            h: self.size,
        }
    }
}

struct Player {
    radius: f32,
    x: f32,
    y: f32,
    color: Color,
}

impl Player {
    fn collides_with(&self, other: &Square) -> bool {
        self.circ().overlaps_rect(&other.rect())
    }

    fn circ(&self) -> Circle {
        Circle {
            x: self.x,
            y: self.y,
            r: self.radius,
        }
    }
}

#[macroquad::main("My game")]
async fn main() {
    rand::srand(miniquad::date::now() as u64);
    const MOVEMENT_SPEED: f32 = 200.0;
    const CIRCLE_RADIUS: f32 = 16.0;
    let square_colors: [Color; 3] = [GREEN, LIGHTGRAY, BLUE];

    let mut squares = vec![];
    let mut circle = Player {
        radius: CIRCLE_RADIUS,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        color: BLACK,
    };

    let mut gameover = false;

    loop {
        clear_background(YELLOW);

        if !gameover {
            let delta_time = get_frame_time();
            let movement = MOVEMENT_SPEED * delta_time;

            if is_key_down(KeyCode::Right) {
                circle.x += movement;
            }
            if is_key_down(KeyCode::Left) {
                circle.x -= movement;
            }
            if is_key_down(KeyCode::Down) {
                circle.y += movement;
            }
            if is_key_down(KeyCode::Up) {
                circle.y -= movement;
            }

            circle.x = clamp(
                circle.x,
                0.0 + CIRCLE_RADIUS,
                screen_width() - CIRCLE_RADIUS * 2.0,
            );
            circle.y = clamp(
                circle.y,
                0.0 + CIRCLE_RADIUS,
                screen_height() - CIRCLE_RADIUS * 2.0,
            );

            if rand::gen_range(0, 99) >= 95 {
                let size = rand::gen_range(16.0, 64.0);
                squares.push(Square {
                    size,
                    speed: rand::gen_range(50.0, 150.0),
                    x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                    y: -size,
                    color: *square_colors.choose().unwrap(),
                });
            }

            for square in &mut squares {
                square.y += square.speed * delta_time;
            }

            squares.retain(|square| square.y < screen_height() + square.size);
        }

        draw_circle(circle.x, circle.y, circle.radius, circle.color);
        for square in &squares {
            draw_rectangle(
                square.x - square.size / 2.0,
                square.y - square.size / 2.0,
                square.size,
                square.size,
                square.color,
            );
        }

        if squares.iter().any(|square| circle.collides_with(square)) {
            gameover = true;
        }

        if gameover {
            let text = "GAME OVER!";
            let text_dimensions = measure_text(text, None, 50, 1.0);
            draw_text(
                text,
                screen_width() / 2.0 - text_dimensions.width / 2.0,
                screen_height() / 2.0,
                50.0,
                RED,
            );
        }

        if gameover && is_key_pressed(KeyCode::Space) {
            squares.clear();
            circle.x = screen_width() / 2.0;
            circle.y = screen_height() / 2.0;
            gameover = false;
        }

        next_frame().await
    }
}
