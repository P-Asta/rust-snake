mod snake;
use std::{thread, time::Duration};

use snake::*;

use snake::enums::Direction;

fn main(){
    let elements = std::sync::Mutex::new(entities::Entities::new());
    thread::scope(|scp|{
        let move_th = scp.spawn(||{
            loop{
                {
                    let mut e = elements.lock().unwrap();
                    e.move_snake();
                    e.map();
                    if e.is_game_over{
                        println!("GAME OVER");
                        break;
                    }
                }
                thread::sleep(Duration::from_millis(100));
            }
        });

        let key_th = scp.spawn(||{
            loop{
                {
                    use crossterm::event::{Event::Key, KeyCode};
                    match crossterm::event::read().unwrap(){
                        Key(key) => {
                            match key.code {
                                KeyCode::Up =>    { let mut e = elements.lock().unwrap(); e.set_dir(Direction::Up)}
                                KeyCode::Down =>  { let mut e = elements.lock().unwrap(); e.set_dir(Direction::Down)}
                                KeyCode::Left =>  { let mut e = elements.lock().unwrap(); e.set_dir(Direction::Left)}
                                KeyCode::Right => { let mut e = elements.lock().unwrap(); e.set_dir(Direction::Right)}
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                {
                    let e = elements.lock().unwrap();
                    if e.is_game_over{
                        break;
                    }
                }
            }

        });

        move_th.join().unwrap();
        key_th.join().unwrap();
    });
    for i in 0..3{
        println!("{}초후에 제실행됩니다", 3 - i);
        thread::sleep(Duration::from_millis(1000));
    }
    main();
}