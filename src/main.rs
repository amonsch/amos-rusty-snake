extern crate sdl2;

use std::time::{Duration, Instant};
use sdl2::rect::Point;
use sdl2::pixels::Color;

mod game;

use game::Game;
use game::component::{Snake, Cherry};


const FLIP_DURATION: Duration = Duration::from_millis(16);
const UPDATE_DURATION: Duration = Duration::from_millis(125);


fn main() {
    let mut game: Game = Game::new("Rusty Snake", 600, 600);

    let snake = Snake::new(30, Color::RGB(75, 139, 190));
    game.components.push(Box::new(snake));

    let cherry = Cherry::new(30, Color::RGB(255, 212, 59));
    game.components.push(Box::new(cherry));

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
