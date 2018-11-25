extern crate sdl2;

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


struct Cherry {
    block: Block,
}

// impl Component for Cherry {

// }


pub struct Snake {
    pub size: i32,
    color: Color,
    pub body: Vec<Point>,
    pub last_valid_directon: Direction,
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
                        if ((part.x + 1) * self.size) as u32 >= state.window_width {
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
                        if ((part.y + 1) * self.size) as u32 >= state.window_height {
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

                    state.direction = self.last_valid_directon.clone();
                    return false;
                } else {
                    self.last_valid_directon = state.direction.clone();
                }
            } else {
                let p = last_pos.unwrap();

                last_pos = Some(Point::new(part.x, part.y));

                part.x = p.x;
                part.y = p.y;
            }
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
            last_valid_directon: Direction::Right,
        }
    }
}
