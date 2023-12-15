use raylib::ffi::Vector2;
use raylib::prelude::*;
use raylib::*;
// mod types;

const W: i32 = 1280;
const H: i32 = 720;
const GRAVITY: f32 = 9.8; // m/p^2
const SQUARE_SIZE: i32 = 10;

static mut offset: Vector2 = Vector2 { x: 0.0, y: 0.0 };

fn initialize() {
    unsafe {
        offset.x = (W%SQUARE_SIZE) as f32;
        offset.y = (H%SQUARE_SIZE) as f32;
    }

}

pub struct ElementSolid{
    mass: f32,
    colour: Color,
    position: Vector2,
}




fn main() {
    let mut instanciated_objects: Vec<ElementSolid> = vec![];

    let (mut rl, thread) = raylib::init()
        .size(W, H)
        .build();

    initialize();



    rl.set_target_fps(20);    

    while !rl.window_should_close() { 
        let delta = rl.get_frame_time();

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
            let mut sand_info = ElementSolid {
                mass: 50.0,
                colour: Color::YELLOW,
                position: Vector2 { x: rl.get_mouse_x() as f32, y: rl.get_mouse_y() as f32 },
            };

            instanciated_objects.push(sand_info);

        }




        
        for thing in instanciated_objects.iter_mut() {
            if thing.position.y < (H - SQUARE_SIZE) as f32 {
                thing.position.y += thing.mass * GRAVITY * delta;
            } 

            let sand: Rectangle = Rectangle::new(thing.position.x, thing.position.y - (thing.position.y % SQUARE_SIZE as f32), SQUARE_SIZE as f32, SQUARE_SIZE as f32);
            
            let mut d = rl.begin_drawing(&thread);
            d.draw_rectangle_rec(&sand, thing.colour);
        }

        
        
        
        
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_fps(10, 10);

    }
}
