mod snake;
#[allow(unused)]
use snake::*;

use snake::entity::Apple;

fn main(){
    let mut e = entities::Entities::new();
    e.apples = vec![Apple{ pos: pos::Pos { x: 0, y: 1 } }];
    e.map();
    std::thread::sleep(std::time::Duration::from_secs(1));


    e.move_snake(enums::Direction::Down);
    e.apples = vec![Apple{ pos: pos::Pos { x: 0, y: 2 } }];
    e.map();
    std::thread::sleep(std::time::Duration::from_secs(1));

    
    e.move_snake(enums::Direction::Down);
    e.apples = vec![Apple{ pos: pos::Pos { x: 0, y: 3 } }];
    e.map();
    std::thread::sleep(std::time::Duration::from_secs(1));




    e.move_snake(enums::Direction::Right);
    e.map();
    std::thread::sleep(std::time::Duration::from_secs(1));
    
    
    e.move_snake(enums::Direction::Right);
    e.map();
    std::thread::sleep(std::time::Duration::from_secs(1));
    
    e.move_snake(enums::Direction::Right);
    e.map();
    std::thread::sleep(std::time::Duration::from_secs(1));

}