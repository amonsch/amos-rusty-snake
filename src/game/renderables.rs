
extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};
use sdl2::video::Window;
use sdl2::render::Canvas;

use game::{Direction, State};


pub trait Component {
    fn update(&mut self, _state: &mut State) {
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


#[derive(Debug)]
pub struct Snake {
    pub size: i32,
    color: Color,
    pub body: Vec<Point>,
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

    fn update(&mut self, state: &mut State) {
        let mut last_pos: Option<sdl2::rect::Point> = None;
        for part in self.body.iter_mut() {
            match last_pos {
                None => {
                    last_pos = Some(sdl2::rect::Point::new(part.x, part.y));
                    match state.direction {
                        Direction::Right => {
                            part.x += 1;
                            if ((part.x + 1) * self.size) as u32 >= state.window_width {
                                println!("x coord: {}", part.x * self.size);
                                panic!("Snake hit right wall - game over!")
                            }
                        },
                        Direction::Left  => {
                            part.x -= 1;
                            if part.x < 0 {
                                panic!("Snake hit left wall - game over!")
                            }
                        },
                        Direction::Down  => {
                            part.y += 1;
                            if ((part.y + 1) * self.size) as u32 >= state.window_height {
                                println!("y coord: {}", part.y * self.size);
                                panic!("Snake hit bottom wall - game over!");
                            }
                        },
                        Direction::Up    => {
                            part.y -= 1;
                            if part.y < 0 {
                                panic!("Snake hit top wall - game over!")
                            }
                        },
                    }
                },
                Some(pos) => {
                    last_pos = Some(sdl2::rect::Point::new(part.x, part.y));
                    part.x = pos.x;
                    part.y = pos.y;
                },
            }
        }
        state.occupied = self.body.clone();

        for (idx, part) in self.body.iter().enumerate() {
            if idx == 0 {
                continue;
            }
            if self.body[0] == *part {
                panic!("Snake hit snake - game over!");
            }
        }


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
        }
    }
}
