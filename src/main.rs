extern crate sdl2;

use std::time::{Duration, Instant};

use sdl2::pixels::Color;

mod game;

use game::Game;
use game::component::Snake;


const FLIP_DURATION: Duration = Duration::from_millis(16);
const UPDATE_DURATION: Duration = Duration::from_millis(200);


fn main() {
    let mut game: Game = Game::new(
        "Rusty Snake",
        500,
        500,
    );

    let snake = Snake::new(30, Color::RGB(0, 255, 0));

    game.components.push(Box::new(snake));

    // let cherry = Cherry::new();
    // game.components.push(Box::new(cherry));

    let mut update_now = Instant::now();
    let mut flip_now = Instant::now();

    while !game.state.quit {
        game.process_input();

        if flip_now.elapsed() > FLIP_DURATION {
            game.canvas.set_draw_color(game.draw_color);
            game.canvas.clear();

            for component in game.components.iter_mut() {
                component.render(&mut game.canvas);
            }

            game.canvas.present();
            flip_now = Instant::now();
        }

        if update_now.elapsed() > UPDATE_DURATION {
            let mut reupdate = false;
            for component in game.components.iter_mut() {
                if !component.update(&mut game.state) {
                    reupdate = true;
                }
            }

            if !reupdate {
                update_now = Instant::now();
            }
        }
    }
}
