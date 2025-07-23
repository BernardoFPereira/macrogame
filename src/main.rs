use macroquad::prelude::*;

#[derive(Clone, Copy)]
struct Position {
    x: f32,
    y: f32,
}

struct Player {
    pos: Position,
    radius: f32,
}

#[macroquad::main(window_config)]
async fn main() {
    let speed = 10.0;

    let mut player = Player {
        pos: Position {
            x: screen_width() / 2.0,
            y: screen_height() / 2.0,
        },
        radius: 16.0,
    };

    loop {
        clear_background(DARKPURPLE);
        draw_text("Macro WIN", 100.0, 50.0, 50.0, WHITE);

        // Capture input
        if is_key_down(KeyCode::Up) {
            player.pos.y -= speed;
        }
        if is_key_down(KeyCode::Down) {
            player.pos.y += speed;
        }
        if is_key_down(KeyCode::Right) {
            player.pos.x += speed;
        }
        if is_key_down(KeyCode::Left) {
            player.pos.x -= speed;
        }

        // Draw Player
        draw_circle(player.pos.x, player.pos.y, player.radius, YELLOW);

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
