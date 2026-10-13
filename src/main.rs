extern crate sdl3;

use sdl3::pixels::Color;
use sdl3::render::FPoint;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;

fn get_neighbours(map:[[bool;MAP_SIZE];MAP_SIZE],y:usize,x:usize)->i32{
    let mut number:i32 = 0;
    let lower:usize = 0;
    let upper:usize = 160;
    if (x > lower && y > lower) && (x < upper && y < upper) {
        if map[y][x-1]  {number+=1;}
        if map[y][x+1]  {number+=1;}
        if map[y-1][x-1]{number+=1;}
        if map[y-1][x]  {number+=1;}
        if map[y-1][x+1]{number+=1;}
        if map[y+1][x-1]{number+=1;}
        if map[y+1][x]  {number+=1;}
        if map[y+1][x+1]{number+=1;}
    }
    return number;
}
static SCREEN_SIZE:f32 = 5.0;
static MAP_SIZE:usize = (800/SCREEN_SIZE as i32) as usize;

static live:Color = Color::RGB(255,255,255);
static dead:Color = Color::RGB(0,0,0);

fn main() {
    let mut counter:i32 = 0;
    let mut generation:u128 = 0;
    let mut exit = false;
    let context = sdl3::init().unwrap();
    let screen = context.video().unwrap();
    let mut canvas = screen.window("GOL",800,800).position_centered().build().unwrap().into_canvas();
    let mut pump =  context.event_pump().unwrap();

    let _ = canvas.set_scale(SCREEN_SIZE,SCREEN_SIZE);
    let mut map:[[bool;MAP_SIZE];MAP_SIZE] = [[false; MAP_SIZE]; MAP_SIZE];
    let mut next_map:[[bool;MAP_SIZE];MAP_SIZE] = map;

    let _ = canvas.present();
    canvas.set_draw_color(live);

    //placement phase
    loop{
        for i in pump.poll_iter(){
            match i{
                Event::Quit{..} => {},
                Event::KeyDown { keycode: Some(Keycode::Escape),..} => {std::process::exit(0);},
                Event::KeyDown { keycode: Some(Keycode::R),..} => {exit=true;},
                _ => {}
            }
        }
        let placement = FPoint{
            x:(pump.mouse_state().x()/SCREEN_SIZE).floor() as f32,
            y:(pump.mouse_state().y()/SCREEN_SIZE).floor() as f32,
        };

        if pump.mouse_state().left(){
            next_map[placement.y as usize][placement.x as usize] = true;
            let _ =  canvas.draw_point(placement);
            let _ =canvas.present();
        }
        if exit {break;}
    }
    println!("started running");
    //simulation
    loop{
        map = next_map;
            for y in 1..MAP_SIZE-1 {
                for x in 1..MAP_SIZE-1 {
                    let n:i32 = get_neighbours(map,y,x);
                    //println!("{} {} at {};{}",n,map[y][x],y,x);
                    //ruleset

                    //s part
                    if map[y][x]&&(n<2||n>3){next_map[y][x] = false;}
                    //b part
                    if !map[y][x] &&    n == 3 {next_map[y][x]=true;}

                    //graphics
                    if map[y][x]{
                        canvas.set_draw_color(live);
                        let _ = canvas.draw_point(FPoint{x:x as f32,y:y as f32,});
                    }

                    if !map[y][x]{
                        canvas.set_draw_color(dead);
                        let _ = canvas.draw_point(FPoint{x:x as f32,y:y as f32,});
                    }
                    counter+=1
                }
            }
        canvas.present();
        generation+=1;
        println!("generation: {}",generation);
        println!("{} pixels iterated",counter);
        counter = 0;
    }
}
