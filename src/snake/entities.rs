use super::entity::*;
use super::enums::*;
use super::pos::Pos;

pub struct Entities{
    pub snakes: Vec<Snake>,
    pub apples: Vec<Apple>
}

#[allow(unused)]
impl Entities{
    pub fn new() -> Self{
        Self {
            snakes: vec![Snake{ pos: Pos{ x: 0, y: 0 } }],
            apples: vec![]
        }
    }
    pub fn move_snake(&mut self, d: Direction){
        let mut old_snakes = self.snakes.clone();
        let pos = match d {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0)
        };
        let mut old_snake_pos = self.snakes[0].pos.clone() + pos;
        for snake in &mut self.snakes{
            let sn = snake.clone();
            snake.pos = old_snake_pos.clone();
            old_snake_pos = sn.clone().pos;
        }
        let mut index = (0, false);
        for apple in &self.apples{
            if apple.pos == self.snakes[0].pos{
                index.1 = true;
                break
            }
            index.0 += 1;
        }
        if index.1{
            self.apples.remove(index.0);
            old_snakes.insert(0, self.snakes[0].clone());
            self.snakes = old_snakes.clone();
        }
    }

    pub fn map(&self){
        println!("");

        let mut m = [[0;17];17];
        for apple in &self.apples{
            if apple.pos.y < 0 || apple.pos.x < 0{ continue; }
            m[apple.pos.y as usize][apple.pos.x as usize] = 2
        }

        for snake in &self.snakes{
            if snake.pos.y < 0 || snake.pos.x < 0{ continue; }
            m[snake.pos.y as usize][snake.pos.x as usize] = 1
        }

        for i in m{
            for j in i{
                let p = match j{
                    1 => "🟩",
                    2 => "🔴",
                    _ => "⬛"
                };
                print!("{p}");
            }
            println!("")
        }
    }
}

