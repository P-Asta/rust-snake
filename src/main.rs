use std::{fmt::Write};
use rand::{Rng, thread_rng};

fn main(){
    let mut map = Map::new(9);
    // println!("{}", map.encoding());
    map.spown_apple();
    println!("{}", map.encoding());

    let m = Move::UP;
    map.move_snake(m);
    println!("{}", map.encoding());
    
    let m = Move::UP;
    map.move_snake(m);
    println!("{}", map.encoding());
}


#[allow(dead_code)]
struct Map{
    map: Vec<Vec<usize>>,
    size: usize,
    apple: Apple,
    player: Vec<usize>
}

#[allow(dead_code)]
struct Apple{
    count: usize
}

#[allow(dead_code)]
pub enum Move {
    UP,
    DOWN,
    LEFT,
    RIGHT
}


#[allow(dead_code)]

impl Map{
    fn new(size: usize) -> Self{
        // let a = vec![vec!["a"]];
        let mut map: Vec<Vec<usize>> = vec![vec![]];
        let mut player = vec![];
        for i in 0..size{
            let mut arr: Vec<usize> = vec![];
            for j in 0..size{
                if i == size/2 && j == size/2{
                    arr.push(2);
                    player = vec![i+1, j];
                }else{
                    arr.push(0)
                }
            }
            map.push(arr);
        }
        Self {
            map: map,
            size: size,
            apple: Apple { count: (10) },
            player: player
        }
    }

    fn spown_apple(&mut self){
        let mut rng = thread_rng();
        self.map[rng.gen_range(0..self.size)][rng.gen_range(0..self.size)] = 3
    }

    fn encoding(&mut self) -> String{
        let mut s: String = String::new();
        for arr in &self.map{
            for i in arr{
                let c = match i {
                    0 => '⬛',
                    1 => '🟩',
                    2 => '🟢',
                    3 => '🔴',
                    _ => '?'
                };
                _ = s.write_char(c);
            }
            s += "\n";
        }
        s
    }

    fn move_snake(&mut self, m: Move){
        self.map[self.player[0]][self.player[1]] = 0;
        match m{
            Move::UP => {
                self.player[0] -= 1
            }
            Move::DOWN => {
                self.player[0] += 1
            }
            Move::LEFT => {
                self.player[1] -= 1
            }
            Move::RIGHT => {
                self.player[1] += 1
            }
        }
        self.map[self.player[0]][self.player[1]] = 2;
    }


}