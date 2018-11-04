extern crate sdl2;

use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::video::Window;
use sdl2::render::Canvas;


const FLIP_DURATION: Duration = Duration::from_millis(16);
const UPDATE_DURATION: Duration = Duration::from_millis(100);

struct Block {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    color: Color,
}


enum Direction {
    Left,
    Right,
    Up,
    Down,
}


trait Renderable {
    fn render(&self, canvas: &mut Canvas<Window>);
}


impl Renderable for Block {
    fn render(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(self.color);
        canvas.fill_rect(Rect::new(self.x, self.y, self.width, self.height)).unwrap();
    }
}


impl Renderable for Snake {
    fn render(&self, canvas: &mut Canvas<Window>) {
        for part in self.body.iter() {
            let block = Block {
                x: part.x * self.size,
                y: part.y * self.size,
                width: self.size as u32,
                height: self.size as u32,
                color: self.color
            };

            block.render(canvas);
        }
    }
}


struct Game{
    window_width: u32,
    window_height: u32,
    canvas: sdl2::render::Canvas<Window>,
    event_pump: sdl2::EventPump,
}


impl Game {
    fn new(title: &str, window_width: u32, window_height: u32) -> Game {
        let context = sdl2::init().unwrap();
        let video = context.video().unwrap();

        let window = video.window(title, window_width, window_height)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window
            .into_canvas()
            .accelerated()
            .build()
            .unwrap();

        let event_pump = context.event_pump().unwrap();

        Game {
            window_width: window_width,
            window_height: window_height,
            canvas: canvas,
            event_pump: event_pump,
        }
    }

    fn render<T: Renderable>(&mut self, renderable: &T) {
        renderable.render(&mut self.canvas);
    }
}


struct Snake {
    size: i32,
    color: Color,
    body: Vec<sdl2::rect::Point>,
}


impl Snake {
    fn new(size: i32, color: Color) -> Snake {
        Snake {
            size: size,
            color: color,
            body: vec![
                sdl2::rect::Point::new(0, 0),
                sdl2::rect::Point::new(1, 0),
                sdl2::rect::Point::new(2, 0),
                sdl2::rect::Point::new(3, 0),
                sdl2::rect::Point::new(4, 0),
            ],
        }
    }

    fn crawl(&mut self, direction: &Direction) {
        let mut last_pos: Option<sdl2::rect::Point> = None;
        for part in self.body.iter_mut() {
            match last_pos {
                None => {
                    last_pos = Some(sdl2::rect::Point::new(part.x, part.y));
                    match direction {
                        Direction::Right => { part.x += 1 },
                        Direction::Left  => { part.x -= 1 },
                        Direction::Down  => { part.y += 1 },
                        Direction::Up    => { part.y -= 1 },
                    }
                },
                Some(pos) => {
                    last_pos = Some(sdl2::rect::Point::new(part.x, part.y));
                    part.x = pos.x;
                    part.y = pos.y;
                },
            }
        }
    }
}


fn main() {
    let mut game: Game = Game::new("Foo", 500, 500);
    let mut snake = Snake::new(25, Color::RGB(0, 255, 0));

    let mut update_now = Instant::now();
    let mut flip_now = Instant::now();
    let mut direction = Direction::Right;
    'running: loop {
        for event in game.event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    println!("Quit Event occured - quitting");
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    println!("Escape pressed - quitting");
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::Left), ..} => {
                    if snake.body[0].x > 0 {
                        direction = Direction::Left;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Right), ..} => {
                    if snake.body[0].x < (game.window_width - snake.size as u32) as i32 {
                        direction = Direction::Right;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Down), ..} => {
                    if snake.body[0].y < (game.window_height - snake.size as u32) as i32 {
                        direction = Direction::Down;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Up), ..} => {
                    if snake.body[0].y > 0 {
                        direction = Direction::Up;
                    }
                },
                _ => {},
            }
        }

        if update_now.elapsed() > UPDATE_DURATION {
            snake.crawl(&direction);
            update_now = Instant::now();
        }

        if flip_now.elapsed() > FLIP_DURATION {
            game.canvas.set_draw_color(Color::RGB(211, 211, 211));
            game.canvas.clear();
            game.render(&snake);
            game.canvas.present();
            flip_now = Instant::now();
        }
    }
}
