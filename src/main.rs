use macroquad::{prelude::*, rand::ChooseRandom};

#[derive(Clone, Copy)]
struct Position {
    x: f32,
    y: f32,
}
impl Position {
    fn screen_center() -> Self {
        Self {
            x: screen_width() / 2.0,
            y: screen_height() / 2.0,
        }
    }
}

struct Player {
    pos: Position,
    radius: f32,
    color: Color,
}
impl Player {
    fn new() -> Self {
        Self {
            pos: Position::screen_center(),
            radius: 16.0,
            color: YELLOW,
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
    fn new_at_rand_pos() -> Self {
        let size = rand::gen_range(16.0, 64.0);
        Self {
            size,
            speed: rand::gen_range(50.0, 150.0),
            color: *vec![GREEN, BLUE].choose().unwrap_or(&GREEN),
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
    let square_colors = Vec::from_iter([GREEN, BLUE, YELLOW]);

    loop {
        clear_background(DARKPURPLE);
        draw_text("Macro WIN", 100.0, 50.0, 50.0, WHITE);

        // Capture input
        let delta_time = get_frame_time();
        if is_key_down(KeyCode::Up) {
            player.pos.y -= MOVE_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Down) {
            player.pos.y += MOVE_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Right) {
            player.pos.x += MOVE_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Left) {
            player.pos.x -= MOVE_SPEED * delta_time;
        }

        // Keep player in bounds
        player.pos.x = clamp(player.pos.x, player.radius, screen_width() - player.radius);
        player.pos.y = clamp(player.pos.y, player.radius, screen_height() - player.radius);

        // Draw Player
        draw_circle(player.pos.x, player.pos.y, player.radius, player.color);

        if rand::gen_range(0, 99) >= 95 {
            squares.push(Shape::new_at_rand_pos());
        }

        // Move squares
        for square in &mut squares {
            square.y += square.speed * delta_time;
        }

        // Cleanup out of bound squares
        squares.retain(|square| square.y < screen_height() + square.size);

        // Move squares
        for square in &squares {
            draw_rectangle(
                square.x - square.size / 2.0,
                square.y - square.size / 2.0,
                square.size,
                square.size,
                square.color,
            )
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
