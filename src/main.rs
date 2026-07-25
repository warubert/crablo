#![allow(dead_code)]
use macroquad::prelude::*;

const MAP: usize = 20;
const TILE_SIZE: (f32, f32) = (32.0, 16.0);

enum AppState {
    Menu,
    Playing,
    GameOver
}

struct Game {}

impl Game {
    fn new() -> Self {
        Game {}
    }

    fn update(&mut self, _dt: f32) -> bool {
        // Update game logic here
        if is_key_pressed(KeyCode::Space){
            return true;
        }
        false
    }

    fn draw(&self) {
        // Draw game here
        draw_text("Game is running", 20.0, 40.0, 30.0, BLACK);
        draw_text("Press Space to die...", 20.0, 80.0, 30.0, DARKGRAY);
    }
}

#[macroquad::main("Crablo")]
async fn main() {
    let mut game = Game::new();
    let mut state = AppState::Menu;

    loop {
        clear_background(WHITE);

        match state {
            AppState::Menu => {
                draw_text("Menu - Enter to start", 100., 100., 40., BLACK);
                if is_key_pressed(KeyCode::Enter) {
                    game = Game::new();
                    state = AppState::Playing;
                }
            }
            AppState::Playing => {
                if game.update(get_frame_time()) {
                    state = AppState::GameOver;
                }
                game.draw();
            }
            AppState::GameOver => {
                game.draw();
                draw_rectangle(0., 0., screen_width(), screen_height(), Color::new(1.,1., 1., 0.7));
                draw_text("GAME OVER", 100., 100., 60., RED);
                draw_text("Enter to reset", 100., 150., 20., GRAY);

                if is_key_pressed(KeyCode::Enter){
                    state = AppState::Menu;
                }
            }
        }

        next_frame().await;
    }

}
