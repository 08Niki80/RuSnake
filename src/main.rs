use macroquad::color::{GREEN, WHITE, RED};

use macroquad::input::{KeyCode, is_key_pressed};

use macroquad::shapes::draw_rectangle;
use macroquad::telemetry::scene_allocated_memory;
use macroquad::text::draw_text;
use macroquad::time::get_frame_time;
use macroquad::window::{clear_background, next_frame, screen_height, screen_width};
use std::collections::VecDeque;
use rand::Rng;



struct Snake {
	direction : (f32, f32),
	body :  VecDeque<(f32, f32)>,
	head : (f32, f32),
	apple: (f32, f32),
	score:  i32,
	frame: (f32,f32),

}


const CELL: f32 = 25.;
const STEP: f32 = 0.5;

impl Snake {
	fn new() -> Self {
		Snake {
			direction : (0., 25.),
			body  : VecDeque::new(),
			head  : (0., 0.),
			apple : (0., 0.),
			score :  0,
			frame : (0., 0.)
		}
	}

	fn add_point(&mut self) { self.score += 1; }

	fn new_apple(&mut self) {
		let mut rng = rand::thread_rng(); 

		self.apple.0 = (((rng.gen_range(0 .. self.frame.0 as i32) )/25)*25) as f32 ;
		self.apple.1 = (((rng.gen_range(25.. self.frame.1 as i32) )/25)*25) as f32 ;
	}
}

#[macroquad::main("snake")]
async fn main(){

	let mut snake = Snake::new();
	let mut chrono = 0. ;

	snake.frame = (screen_width(), screen_height()); 
	snake.new_apple();

	snake.body.push_back(snake.head);

	draw_text(snake.score.to_string(),snake.head.0, snake.head.1, 10.0, GREEN);

	draw_rectangle(snake.head.0 , snake.head.1 , CELL, CELL, GREEN);
	draw_rectangle(snake.apple.0, snake.apple.1, CELL, CELL, RED  );

	loop{

		chrono += get_frame_time();

		if chrono >= STEP{
			chrono -= STEP;

			snake.head.0 += snake.direction.0;
			snake.head.1 += snake.direction.1;
			
			
			snake.body.push_back(snake.head);
		}

		snake.frame = (screen_width(), screen_height()); 

		if is_key_pressed(KeyCode::Right) { snake.direction.0 =  CELL; snake.direction.1 =  0.  ; }
		if is_key_pressed(KeyCode::Down ) { snake.direction.0 =  0.  ; snake.direction.1 =  CELL; }
		if is_key_pressed(KeyCode::Up   ) { snake.direction.0 =  0.  ; snake.direction.1 = -CELL; }
		if is_key_pressed(KeyCode::Left ) { snake.direction.0 = -CELL; snake.direction.1 =  0.  ; }
		
		clear_background(WHITE);

		if snake.head == snake.apple {
			snake.add_point();
			snake.new_apple();
		}
		else if snake.body.len() > snake.score as usize +1 {
			snake.body.pop_front();
		}

		draw_text     (snake.score.to_string(),50., 50., 10.0, GREEN);

		draw_rectangle(snake.apple.0 , snake.apple.1, CELL, CELL, RED  );
		for block in &snake.body{
			draw_rectangle(block.0  , block.1 , CELL, CELL, GREEN);
		}

		
		
		

		next_frame().await;

		
	}
}



