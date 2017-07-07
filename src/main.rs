#![windows_subsystem = "windows"]
extern crate sdl2;
extern crate rand;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use rand::Rng;
use std::{thread,time};

struct Segment {
    x: i32,
    y: i32,
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    
    let window = video_subsystem.window("Snake",480,360).opengl().build().unwrap();
    
    let mut canvas = window.into_canvas().build().unwrap();
    
    let mut event_pump = sdl_context.event_pump().unwrap();
    
    let mut snake_x: i32 = 1;
    let mut snake_y: i32 = 1;
    let mut snake_dir: i32 = 3;
    let mut tail: Vec<Segment> = Vec::new();
    let mut tail_length: usize = 3;
    
    let mut fruit: Segment = Segment {x:10,y:10};
    
    'main: loop {
        for event in event_pump.poll_iter() {
            let old_dir = snake_dir.clone();
            match event {
                Event::Quit {..} => break 'main,
                Event::KeyDown {keycode: Some(Keycode::W), ..} => {
                    if old_dir != 1 {
                        snake_dir=0;
                    }
                },
                Event::KeyDown {keycode: Some(Keycode::S), ..} => {
                    if old_dir != 0 {
                        snake_dir=1;
                    }
                },
                Event::KeyDown {keycode: Some(Keycode::A), ..} => {
                    if old_dir != 3 {
                        snake_dir=2;
                    }
                },
                Event::KeyDown {keycode: Some(Keycode::D), ..} => {
                    if old_dir != 2 {
                        snake_dir=3;
                    }
                },
                _ => {},
            }
        }
        if snake_x == -1 || snake_x == 33 {
            break 'main;
        } else if snake_y == -1 || snake_y == 25 {
            break 'main;
        }
        for segment in tail.iter() {
            if (segment.x == snake_x) && (segment.y == snake_y) {
                break 'main;
            }
        }
        if (snake_x == fruit.x) && (snake_y == fruit.y) {
            tail_length+=1;
            fruit = Segment{x: rand::thread_rng().gen_range(0,32), y: rand::thread_rng().gen_range(0,24)};
        }
        tail.push(Segment {x: snake_x, y: snake_y});
        if tail.len() > tail_length {
            tail.remove(0);
        }
        canvas.set_draw_color(Color::RGB(255,255,255));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(0,255,0));
        canvas.fill_rect(Rect::new(snake_x*15,snake_y*15,15,15));
        for segment in tail.iter() {
            canvas.fill_rect(Rect::new(segment.x*15,segment.y*15,15,15));
        }
        canvas.set_draw_color(Color::RGB(255,0,0));
        canvas.fill_rect(Rect::new(fruit.x*15,fruit.y*15,15,15));
        canvas.present();
        match snake_dir {
            0 => {snake_y-=1;},
            1 => {snake_y+=1;},
            2 => {snake_x-=1;},
            3 => {snake_x+=1;},
            _ => {},
        }
        thread::sleep(time::Duration::from_millis(75));
    }
}