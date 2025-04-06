use macroquad::{miniquad::window::screen_size, prelude::*, rand::ChooseRandom};
use std::fs;

struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
    color: Color,
    collided: bool,
}

impl Shape {
    fn collides_with(&self, other: &Self) -> bool {
        self.rect().overlaps(&other.rect())
    }

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
    fn collides_with(&self, other: &Shape) -> bool {
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

    let mut score: u32 = 0;
    let mut high_score: u32 = fs::read_to_string("highscore.dat")
        .map_or(Ok(0), |i| i.parse::<u32>())
        .unwrap_or(0);
    let old_high_score = high_score;

    let mut squares = vec![];
    let mut bullets: Vec<Shape> = vec![];
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

            if is_key_pressed(KeyCode::Space) {
                bullets.push(Shape {
                    x: circle.x,
                    y: circle.y,
                    speed: MOVEMENT_SPEED * 2.0,
                    size: 5.0,
                    color: ORANGE,
                    collided: false,
                })
            }

            if rand::gen_range(0, 99) >= 95 {
                let size = rand::gen_range(16.0, 64.0);
                squares.push(Shape {
                    size,
                    speed: rand::gen_range(50.0, 150.0),
                    x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                    y: -size,
                    color: *square_colors.choose().unwrap(),
                    collided: false,
                });
            }

            for square in &mut squares {
                square.y += square.speed * delta_time;
            }
            squares.retain(|square| square.y < screen_height() + square.size);
            squares.retain(|square| !square.collided);

            for bullet in &mut bullets {
                bullet.y -= bullet.speed * delta_time
            }
            bullets.retain(|bullet| bullet.y < screen_height() + bullet.size);
            squares.retain(|bullet| !bullet.collided);
        }

        for bullet in &bullets {
            draw_circle(bullet.x, bullet.y, bullet.size, bullet.color);
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

        draw_text(format!("Score {}", score).as_str(), 10.0, 35.0, 25.0, BLACK);
        let highscore_text = format!("High score: {}", high_score);
        let text_dimension = measure_text(highscore_text.as_str(), None, 24, 1.0);
        draw_text(
            highscore_text.as_str(),
            screen_width() - text_dimension.width - 10.0,
            35.0,
            25.0,
            BLACK,
        );

        if squares.iter().any(|square| circle.collides_with(square)) {
            if score == high_score {
                fs::write("highscore.dat", high_score.to_string()).ok();
            }
            gameover = true;
        }

        for square in squares.iter_mut() {
            for bullet in bullets.iter_mut() {
                if bullet.collides_with(square) {
                    bullet.collided = true;
                    square.collided = true;
                    score += square.size.round() as u32 / 2;
                    high_score = high_score.max(score);
                }
            }
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
            if high_score > old_high_score {
                let hs_text = format!("NEW HIGH SCORE: {}", high_score);
                let hs_text_dimensions = measure_text(hs_text.as_str(), None, 50, 1.0);
                draw_text(
                    hs_text.as_str(),
                    screen_width() / 2.0 - hs_text_dimensions.width / 2.0,
                    screen_height() / 2.0 + text_dimensions.height + 5.0,
                    50.0,
                    RED,
                );
            }
        }

        if gameover && is_key_pressed(KeyCode::Space) {
            squares.clear();
            bullets.clear();
            circle.x = screen_width() / 2.0;
            circle.y = screen_height() / 2.0;
            gameover = false;
            score = 0
        }

        next_frame().await
    }
}
