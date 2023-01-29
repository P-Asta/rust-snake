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
    pub direction: Direction
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
            direction: Direction::Null
        }
    }
    pub fn move_snake(&mut self){
        let d = self.direction;
        let mut old_snakes = self.snakes.clone();
        let pos = match d {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            _ => (0, 0)
        };
        let mut new_snake = self.snakes[0].pos.clone() + pos;
        if self.snakes.contains(&Snake{pos: new_snake.clone()}){
            return;
        }
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
            if snake.pos.y < 0 || snake.pos.x < 0 || 
                snake.pos.y > 16 || snake.pos.x > 16
                    { continue; }
            m[snake.pos.y as usize][snake.pos.x as usize] = 1
        }

        for i in m{
            for j in i{
                let null = Color::Rgb { r: 10, g: 10, b: 10 };
                let color = match j{
                    1 => Color::Rgb { r: 0, g: 240, b: 0 },
                    2 => Color::Rgb { r: 160, g: 0, b: 0 },
                    _ => null
                };

                execute!(
                    stdout(),
                    SetForegroundColor(color),
                    SetBackgroundColor(color),
                    Print("ㅤ".to_string()),
                    ResetColor
                ).unwrap();

            }
            print!("\n")
        }
    }
}

