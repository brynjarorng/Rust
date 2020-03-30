/*
This is Huldars attempt at creating a game of snake using the programming language Rust
Huldar is a total noob but will try his best ^_^
*******************************************************************************************
Creation date: 26.3.2020 at 19:40 (UTC+1)
Author: Huldar
License: Anybody can do pretty much whatever they want with this code.
*******************************************************************************************

How it will work.
You run the program and the _snake_ will start moving.
The snake can go through a wall on the left side and emerge from the right side
Every now and then an "apple" will appear on the screen which if the snake "eats" it, then the snake will grow by 1 "step"
The user can use the arrow keys to move the snake around.
As the snake "eats" more "apples" he not only grows but also gains speed.
The game is over when the snake touches itself or if there is no more space on the screen for the snake.
*/

//Using windows. This command avoids opening the command prompt on execution
//#![windows_subsystem = "windows"]

//External crates
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

//Used dependencies
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use rand::Rng;

//Something some youtuber said would help with making the snake grow. Will look into:
//use std::collections::LinkedList;


//Declaring constants
const X_MAX: i32 = 31;
const Y_MAX: i32 = 23;
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

#[derive(Clone, PartialEq)]
enum Direction {
    Right, Left, Up, Down
}

//This is the game struct
struct Game {
    gl: GlGraphics,
    snake: Snake,
    apple: Apple,
    updates_per_second: u64,
}

impl Game {
    fn render(&mut self, arg: &RenderArgs) {
        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear(BLACK,gl);
        });

        self.apple.render(arg);
        self.snake.render(arg);
    }

    fn update(&mut self) {
        self.snake.update();

        //Eat apple?
        if self.snake.pos_x == self.apple.pos_x && self.snake.pos_y == self.apple.pos_y {
            self.snake.grow();
            self.apple.update();
        }
    }

    //The function that is called when a button is pressed, changing the direction of the snake
    fn pressed(&mut self, btn: &Button) {
        let last_direction = self.snake.dir.clone();

        self.snake.dir = match btn {
            &Button::Keyboard(Key::Up)
                if last_direction != Direction::Down => Direction::Up,
            &Button::Keyboard(Key::Down)
                if last_direction != Direction::Up => Direction::Down,
            &Button::Keyboard(Key::Left)
                if last_direction != Direction::Right => Direction::Left,
            &Button::Keyboard(Key::Right)
                if last_direction != Direction::Left => Direction::Right,

                //We default to the last direction so if somebody accidentally presses a different button, nothing happens
               _ => last_direction
            }
    }
}

struct Snake {
    gl: GlGraphics,
    pos_x: i32,
    pos_y: i32,
    dir: Direction,
    length: u64,
}

impl Snake {
    pub fn render(&mut self, args: &RenderArgs) {
        let square = graphics::rectangle::square((self.pos_x*20) as f64, (self.pos_y*20) as f64, 20_f64);

        self.gl.draw(args.viewport(), |c,gl|{
            let transform = c.transform;

            graphics::rectangle(WHITE, square, transform, gl)
        });
    }

    //Step function
    //Maximum position in window for pos_x is 31 and maximum position in window for pos_y is 23
    fn update(&mut self) {
        match self.dir {
            Direction::Left => self.pos_x = ((self.pos_x-1)%(X_MAX+1)+(X_MAX+1))%(X_MAX+1),      //Weird remainder calculations to change the % operator into modulus so we can leave from one side of the screen and enter on another side
            Direction::Right => self.pos_x = ((self.pos_x+1)%(X_MAX+1)+(X_MAX+1))%(X_MAX+1),
            Direction::Up => self.pos_y = ((self.pos_y-1)%(Y_MAX+1)+(Y_MAX+1))%(Y_MAX+1),
            Direction::Down => self.pos_y = ((self.pos_y+1)%(Y_MAX+1)+(Y_MAX+1))%(Y_MAX+1),
        }
    }

    //Growing snake
    fn grow (&mut self) {
        self.length += 1;
        println!("Snake length is {}", self.length);
    }
}

//Apple struct
struct Apple {
    gl: GlGraphics,
    pos_x: i32,
    pos_y: i32,
}

impl Apple {
    //render fn
    fn render(&mut self, args: &RenderArgs) {
        let square = graphics::rectangle::square((self.pos_x*20) as f64, (self.pos_y*20) as f64, 20_f64);

        self.gl.draw(args.viewport(), |c,gl|{
            let transform = c.transform;

            graphics::rectangle(RED, square, transform, gl)
        });
    }

    fn update (&mut self) {
        let mut rng = rand::thread_rng();
        self.pos_x = rng.gen_range(1, X_MAX);
        self.pos_y = rng.gen_range(1, Y_MAX);
    }
}

/*
*****************************************************************************************************************************
Main function
*/
fn main() {
    //random
    let mut rng = rand::thread_rng();

    //Variable declaration
    
        //Boolean game over?
    
    //Stuff from random youtube video: https://www.youtube.com/watch?v=HCwMb0KslX8
    let opengl = OpenGL::V3_2;

    //WindoSettings::new{Title T (must be string), Size S (must be array, [width,height])}
    //Coordinates for snake: (x0,y0) = (0,0) --> (x1,y1) --> (X_MAX,Y_MAX)
    let mut window: GlutinWindow = WindowSettings::new("Snake Game", (640, 480))
    .fullscreen(false)
    .vsync(true)
    .graphics_api(OpenGL::V3_2)
    .exit_on_esc(true)
    .build()
    .unwrap();

    let mut snake_vec: Vec<Snake> = Vec::new();
    snake_vec.push(Snake {gl: GlGraphics::new(opengl), pos_x: 10, pos_y: 10, dir: Direction::Right, length: 1});

    //Here the core game resides
    let mut game = Game {
        gl: GlGraphics::new(opengl),
//        snake: snake_vec[0],
        snake: Snake {gl: GlGraphics::new(opengl), pos_x: 10, pos_y: 10, dir: Direction::Right, length: 1},
        apple: Apple {gl: GlGraphics::new(opengl), pos_x: rng.gen_range(1,X_MAX), pos_y: rng.gen_range(1, Y_MAX)},
        updates_per_second: 5,
    };


    //Cool screen with "GAME STARTING" + "3" + "2" + "1" "BEGIN"

    //Loop - While Game == notover && make sure the runtime of the loop is controlled by the speed
    let mut events = Events::new(EventSettings::new()).ups(game.updates_per_second);
    while let Some(e) = events.next(&mut window) {

        if let Some(r) = e.render_args() {           //If the event is a render event we will do something.
            game.render(&r);
        }

        if let Some(u) = e.update_args() {             //If the event is an update we wil do something.
            game.update();

            //Change speed of snake depending on how long the snake is.            game.updates_per_second = game.snake.length*5;
        }

        //Change direction when button is pressed event call
        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                game.pressed(&k.button);
            }
        }
  
            //Include if apple was eaten
    
            //Include if crash into self

        //Check if game is over to break loop

    //When game is over make a cool ASCII "artwork" with a GAME OVER or a YOU WON

//    println!("(X,Y) = ({},{})",game.snake.pos_x,game.snake.pos_y);
    }

  //  println!("If this prints out, that means the program is working :)");
}