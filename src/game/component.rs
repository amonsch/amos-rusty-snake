extern crate sdl2;
extern crate rand;

use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};
use sdl2::video::Window;
use sdl2::render::Canvas;

use game::{Direction, State};


pub trait Component {
    fn update(&mut self, _state: &mut State) -> bool {
        return false;
    }

    fn render(&mut self, &mut Canvas<Window>) {
    }
}

struct Block {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub color: Color,
}


impl Component for Block {
    fn render(&mut self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(self.color);
        canvas.fill_rect(Rect::new(self.x, self.y, self.width, self.height)).unwrap();
    }
}


pub struct Cherry {
    pub pos: Point,
    pub size: i32,
    color: Color,
}


impl Component for Cherry {
    fn update(&mut self, state: &mut State) -> bool {
        match state.cherry_pos {
            None => {
                let max_x = state.window_width / ( self.size as u32);
                let max_y = state.window_height / (self.size as u32);
                self.pos.x = (rand::random::<u32>() % max_x) as i32;
                self.pos.y = (rand::random::<u32>() % max_y) as i32;
                state.cherry_pos = Some(self.pos.clone());
            },
            _ => {}
        }
        return true;
    }

    fn render(&mut self, canvas: &mut Canvas<Window>) {
        let mut block  = Block {
            x: self.pos.x * self.size,
            y: self.pos.y * self.size,
            width: self.size as u32,
            height: self.size as u32,
            color: self.color,
        };
        block.render(canvas);
    }
}


pub struct Snake {
    pub size: i32,
    color: Color,
    pub body: Vec<Point>,
    pub moving_direction: Direction,
}


impl Component for Snake {
    fn render(&mut self, canvas: &mut Canvas<Window>) {
        for part in self.body.iter() {
            let mut block  = Block {
                x: part.x * self.size,
                y: part.y * self.size,
                width: self.size as u32,
                height: self.size as u32,
                color: self.color
            };

            block.render(canvas);
        }
    }

    fn update(&mut self, state: &mut State) -> bool{
        let neck = self.body[1].clone();

        let mut last_pos: Option<Point> = None;
        for (idx, part) in self.body.iter_mut().enumerate() {

            if idx == 0 {
                last_pos = Some(Point::new(part.x, part.y));
                match state.direction {
                    Direction::Right => {
                        part.x += 1;
                        if (part.x * self.size) as u32 >= state.window_width {
                            panic!("Snake hit right wall - game over!");
                        }
                    },
                    Direction::Left => {
                        part.x -= 1;
                        if part.x < 0 {
                            panic!("Snake hit left wall - game over!");
                        }
                    },
                    Direction::Down => {
                        part.y += 1;
                        if (part.y * self.size) as u32 >= state.window_height {
                            panic!("Snake hit bottom wall - game over!");
                        }
                    },
                    Direction::Up => {
                        part.y -= 1;
                        if part.y < 0 {
                            panic!("Snake hit top wall - game over!");
                        }
                    },
                }

                // If HEAD has the same coordinates as the NECK
                // revert the position update and mark the update as failed
                if part.x == neck.x && part.y == neck.y {
                    let p =  last_pos.unwrap();
                    part.x = p.x;
                    part.y = p.y;

                    state.direction = self.moving_direction.clone();
                    return false;
                } else {
                    self.moving_direction = state.direction.clone();
                }
            } else {
                let p = last_pos.unwrap();

                last_pos = Some(Point::new(part.x, part.y));

                part.x = p.x;
                part.y = p.y;
            }
        }

        match state.cherry_pos {
            Some(pos) => {
                if self.body[0] == pos {
                    state.cherry_pos = None;
                    self.body.push(last_pos.unwrap());
                }
            },
            _ => {}
        }



        for (idx, part) in self.body.iter().enumerate() {
            if idx == 0 {
                continue;
            }
            if self.body[0] == *part {
                panic!("Snake hit snake - game over!");
            }
        }
        return true;
    }
}


impl Snake {
    pub fn new(size: i32, color: Color) -> Snake {
        Snake {
            size: size,
            color: color,
            body: vec![
                sdl2::rect::Point::new(4, 0),
                sdl2::rect::Point::new(3, 0),
                sdl2::rect::Point::new(2, 0),
                sdl2::rect::Point::new(1, 0),
                sdl2::rect::Point::new(0, 0),
            ],
            moving_direction: Direction::Right,
        }
    }
}

impl Cherry {
    pub fn new(size: i32, color: Color) -> Cherry {
        Cherry {
            pos: Point::new(-1, -1),
            size: size,
            color: color,
        }
    }
}
