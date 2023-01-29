use std::io::stdout;

use crossterm::ExecutableCommand;
use crossterm::execute;
use crossterm::style::Color;
use crossterm::style::Print;
use crossterm::style::ResetColor;
use crossterm::style::SetBackgroundColor;
use crossterm::style::SetForegroundColor;
use rand::{thread_rng, Rng};

use super::entity::*;
use super::enums::*;
use super::pos::Pos;

pub struct Entities{
    pub snakes: Vec<Snake>,
    pub apple: Apple,
    pub direction: Direction,
    pub move_count: usize,
    pub is_game_over: bool,
}

#[allow(unused)]
impl Entities{
    pub fn new() -> Self{
        Self {
            snakes: vec![Snake{ pos: Pos{ x: 8, y: 9 } }],
            apple: Apple {
                pos: Pos {
                    x: Rng::gen_range(&mut thread_rng(), 0..17),
                    y: Rng::gen_range(&mut thread_rng(), 0..17)
                }
            },
            direction: Direction::Null,
            move_count: 1,
            is_game_over: false
        }
    }
    pub fn move_snake(&mut self){
        if self.get_point() < 0{
            self.is_game_over = true; return;
        }
        self.move_count += 1;
        let d = self.direction;
        let mut old_snakes = self.snakes.clone();
        let pos = match d {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            _ => {self.move_count -= 1;(0, 0)}
        };

        let mut new_snake = self.snakes[0].pos.clone() + pos;
        if self.snakes.contains(&Snake{pos: new_snake.clone()}){
            match self.direction {
                Direction::Null => {}
                _ => { self.is_game_over = true; }
            }
            return;
        }
        if new_snake.y < 0 || new_snake.x < 0 || 
                new_snake.y > 16 || new_snake.x > 16
                    { self.is_game_over = true; return; }
        self.snakes.insert(0, Snake{pos: new_snake});

        if self.apple.pos != self.snakes[0].pos{
            self.snakes.remove(self.snakes.len()-1);
        }else{
            self.apple = Apple {
                pos: Pos {
                    x: Rng::gen_range(&mut thread_rng(), 0..17),
                    y: Rng::gen_range(&mut thread_rng(), 0..17)
                }
            }
        }
    }

    pub fn map(&self){
        stdout().execute(crossterm::cursor::MoveTo(0, 0)).unwrap();
        stdout().execute(crossterm::terminal::Clear(crossterm::terminal::ClearType::All)).unwrap();

        let mut m = [[0;17];17];
        m[self.apple.pos.y as usize][self.apple.pos.x as usize] = 2;

        for snake in &self.snakes{
            m[snake.pos.y as usize][snake.pos.x as usize] = 1
        }
        for i in 0..19{
            let c = if i % 2 == 0{
                Color::Rgb { r: 255, g: 255, b: 255 }
            }else{
                Color::Rgb { r: 210, g: 254, b: 255 }
            };
            execute!(
                stdout(),
                SetForegroundColor(c),
                SetBackgroundColor(c),
                Print("ㅤ".to_string()),
                ResetColor
            ).unwrap();
        }
        let mut c_i = 0;
        print!("\n");
        for i in m{
            c_i += 1;
            let c = if c_i % 2 == 0{
                Color::Rgb { r: 255, g: 255, b: 255 }
            }else{
                Color::Rgb { r: 210, g: 254, b: 255 }
            };
            execute!(
                stdout(),
                SetForegroundColor(c),
                SetBackgroundColor(c),
                Print("ㅤ".to_string()),
                ResetColor
            ).unwrap();
            for j in i{
                let null = Color::Rgb { r: 10, g: 10, b: 10 };
                let color = match j{
                    1 => {Color::Rgb { r: 0, g: 240, b: 0 }},
                    2 => Color::Rgb { r: 240, g: 0, b: 0 },
                    _ => null
                };
                if color != null{
                    execute!(
                        stdout(),
                        SetForegroundColor(color),
                        SetBackgroundColor(color),
                        Print("ㅤ".to_string()),
                        ResetColor
                    ).unwrap();
                }else{
                    print!("ㅤ")
                }
            }
            execute!(
                stdout(),
                SetForegroundColor(c),
                SetBackgroundColor(c),
                Print("ㅤ".to_string()),
                ResetColor
            ).unwrap();
            print!("\n");
        }
        for i in 0..19{
            let c = if i % 2 == 0{
                Color::Rgb { r: 255, g: 255, b: 255 }
            }else{
                Color::Rgb { r: 210, g: 254, b: 255 }
            };
            execute!(
                stdout(),
                SetForegroundColor(c),
                SetBackgroundColor(c),
                Print("ㅤ".to_string()),
                ResetColor
            ).unwrap();
        }
        println!("\npoint: {}", self.get_point());
    }

    fn get_point(&self) -> isize{
        self.snakes.len() as isize * 10 - (self.move_count as f64 / 2.0) as isize
    }

    pub fn set_dir(&mut self, d: Direction){
        match &self.direction {
            Direction::Up => { 
                match d {
                    Direction::Down => { return }
                    _ => {}
                }
            }
            Direction::Down => { 
                match d {
                    Direction::Up => { return }
                    _ => {}
                }
            }
            Direction::Left => { 
                match d {
                    Direction::Right => { return }
                    _ => {}
                }
            }
            Direction::Right => { 
                match d {
                    Direction::Left => { return }
                    _ => {}
                }
            }
            _ => {}
        };
        self.direction = d;
    }
}