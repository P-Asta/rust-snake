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
                    thread::sleep(Duration::from_millis(1000));
                }
            }
        });

        let key_th = scp.spawn(||{
            loop{
                {
                    use crossterm::event::{Event::Key, KeyCode};
                    match crossterm::event::read().unwrap(){
                        Key(key) => {
                            match key.code {
                                KeyCode::Up =>    { let mut e = elements.lock().unwrap(); e.direction = Direction::Up}
                                KeyCode::Down =>  { let mut e = elements.lock().unwrap(); e.direction = Direction::Down}
                                KeyCode::Left =>  { let mut e = elements.lock().unwrap(); e.direction = Direction::Left}
                                KeyCode::Right => { let mut e = elements.lock().unwrap(); e.direction = Direction::Right}
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
            }

        });

        move_th.join().unwrap();
        key_th.join().unwrap();
    });

}