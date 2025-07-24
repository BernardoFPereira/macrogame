use std::any::Any;

use macroquad::{prelude::*, rand::ChooseRandom};

struct Player {
    shape: Shape,
}
impl Player {
    fn new() -> Self {
        Self {
            shape: Shape {
                color: YELLOW,
                size: 32.0,
                speed: MOVE_SPEED,
                x: screen_width() / 2.0,
                y: screen_height() / 2.0,
            },
        }
    }
}

struct Shape {
    color: Color,
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
}
impl Shape {
    fn collides_with(&self, other: &Self) -> bool {
        self.rect().overlaps(&other.rect())
    }

    fn circle(&self) -> Circle {
        Circle {
            x: self.x - self.size,
            y: self.y - self.size,
            r: self.size,
        }
    }

    fn rect(&self) -> Rect {
        Rect {
            x: self.x - self.size / 2.0,
            y: self.y - self.size / 2.0,
            w: self.size,
            h: self.size,
        }
    }

    fn new_at_rand_pos() -> Self {
        let size = rand::gen_range(16.0, 64.0);
        Self {
            size,
            speed: rand::gen_range(50.0, 150.0),
            color: *vec![GREEN, BLUE, DARKGREEN, DARKBLUE]
                .choose()
                .unwrap_or(&GREEN),
            x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
            y: -size,
        }
    }
}

const MOVE_SPEED: f32 = 200.0;

#[macroquad::main(window_config)]
async fn main() {
    rand::srand(miniquad::date::now() as u64);

    let mut player = Player::new();
    let mut squares = vec![];
    let mut game_over = false;

    loop {
        clear_background(DARKPURPLE);

        if !game_over {
            // Capture input
            let delta_time = get_frame_time();
            if is_key_down(KeyCode::Up) {
                player.shape.y -= MOVE_SPEED * delta_time;
            }
            if is_key_down(KeyCode::Down) {
                player.shape.y += MOVE_SPEED * delta_time;
            }
            if is_key_down(KeyCode::Right) {
                player.shape.x += MOVE_SPEED * delta_time;
            }
            if is_key_down(KeyCode::Left) {
                player.shape.x -= MOVE_SPEED * delta_time;
            }

            // Keep player in bounds
            player.shape.x = clamp(
                player.shape.x,
                player.shape.size,
                screen_width() - player.shape.size,
            );
            player.shape.y = clamp(
                player.shape.y,
                player.shape.size,
                screen_height() - player.shape.size,
            );

            if rand::gen_range(0, 99) >= 95 {
                squares.push(Shape::new_at_rand_pos());
            }

            // Move squares
            for square in &mut squares {
                square.y += square.speed * delta_time;
            }

            // Draw Player
            draw_circle(
                player.shape.x,
                player.shape.y,
                player.shape.size / 2.0,
                player.shape.color,
            );

            // Draw squares
            for square in &squares {
                draw_rectangle(
                    square.x - square.size / 2.0,
                    square.y - square.size / 2.0,
                    square.size,
                    square.size,
                    square.color,
                )
            }
            // Cleanup out of bound squares
            squares.retain(|square| square.y < screen_height() + square.size);
        }

        if squares
            .iter()
            .any(|square| square.collides_with(&player.shape))
        {
            println!("PLAYER HIT");
        }

        next_frame().await
    }
}

fn window_config() -> Conf {
    Conf {
        window_title: "MacroGame".to_string(),
        window_width: 1080,
        window_height: 720,
        ..Default::default()
    }
}
