#![allow(dead_code)]
use macroquad::prelude::*;

mod entities;
mod game;
mod map;
mod renderer;

use game::Game;

enum AppState {
    Menu,
    Playing,
    GameOver,
}

#[macroquad::main("Crablo")]
async fn main() {
    let mut game = Game::new();
    let mut state = AppState::Menu;

    loop {
        clear_background(WHITE);

        match state {
            AppState::Menu => {
                draw_text("Menu - Enter para começar", 100., 100., 40., BLACK);
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
                draw_rectangle(
                    0.,
                    0.,
                    screen_width(),
                    screen_height(),
                    Color::new(1., 1., 1., 0.7),
                );

                //victory vs defeat logic
                let (msg, color) = if game.hp > 0 {
                    ("VITÓRIA", GOLD)
                } else {
                    ("GAME OVER", RED)
                };

                draw_text(
                    msg,
                    screen_width() / 2. - 100.,
                    screen_height() / 2.,
                    60.,
                    color,
                );
                draw_text(
                    format!("Pontuação: {}", game.score),
                    screen_width() / 2. - 80.,
                    screen_height() / 2. + 50.,
                    30.,
                    BLACK,
                );
                draw_text(
                    "Enter para reiniciar",
                    screen_width() / 2. - 80.,
                    screen_height() / 2. + 90.,
                    20.,
                    GRAY,
                );

                if is_key_pressed(KeyCode::Enter) {
                    state = AppState::Menu;
                }
            }
        }

        next_frame().await;
    }
}
