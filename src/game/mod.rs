
extern crate sdl2;

use sdl2::rect::Point;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::video::Window;

pub mod component;

use game::component::Component;


#[derive(Copy,Clone,Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}


pub struct State {
    pub window_width: u32,
    pub window_height: u32,
    pub direction: Direction,
    pub occupied: Vec<Point>,
    pub cherry_pos: Option<Point>,
    pub quit: bool,
}


pub struct Game {
    pub draw_color: Color,
    pub canvas: sdl2::render::Canvas<Window>,
    pub components: Vec<Box<Component>>,
    pub event_pump: sdl2::EventPump,
    pub state: State,
}


impl Game {
    pub fn new(title: &str, window_width: u32, window_height: u32) -> Game {
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
        let components: Vec<Box<Component>> = Vec::new();
        let occupied: Vec<Point> = Vec::new();

        Game {
            canvas: canvas,
            draw_color: Color::RGB(211, 211, 211),
            event_pump: event_pump,
            components: components,
            state: State {
                quit: false,
                direction: Direction::Right,
                window_width: window_width,
                window_height: window_height,
                occupied: occupied,
                cherry_pos: None,
            },
        }
    }

    pub fn process_input(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    println!("Quit Event occured - quitting");
                    self.state.quit = true;
                },
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    println!("Escape pressed - quitting");
                    self.state.quit = true;
                },
                Event::KeyDown { keycode: Some(Keycode::Left), ..} => {
                    match self.state.direction {
                        Direction::Right => {},
                        _ => {
                            self.state.direction = Direction::Left;
                        }
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Right), ..} => {
                    match self.state.direction {
                        Direction::Left => {},
                        _ => {
                            self.state.direction = Direction::Right;
                        }
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Down), ..} => {
                    match self.state.direction {
                        Direction::Up => {},
                        _ => {
                            self.state.direction = Direction::Down;
                        }
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Up), ..} => {
                    match self.state.direction {
                        Direction::Down => {},
                        _ => {
                            self.state.direction = Direction::Up;
                        }
                    }
                },
                _ => {},
            }
        }
    }
}
