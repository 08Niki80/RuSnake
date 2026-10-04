use macroquad::color::{GREEN, WHITE, RED};

use macroquad::input::{KeyCode, is_key_pressed};

use macroquad::shapes::draw_rectangle;
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
const STEP: f32 = 0.25;

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

	fn new_snake(&mut self) {
		self.head =         ((self.frame.0 as i32 /50*25) as f32,CELL*3.);
		self.body.push_front(self.head);

		self.body.push_back (((self.frame.0 as i32 /50*25) as f32,CELL*2.));
		self.body.push_back (((self.frame.0 as i32 /50*25) as f32,CELL   ));
	}

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
	snake.new_snake();


	loop{
		//nouvelle image
		clear_background(WHITE);
		draw_text     (snake.score.to_string(),50., 50., 10.0, GREEN);

		draw_rectangle(snake.apple.0 , snake.apple.1, CELL, CELL, RED  );
		for block in &snake.body{
			draw_rectangle(block.0  , block.1 , CELL, CELL, GREEN);
		}

		//avancement du serpent
		chrono += get_frame_time();
		
		if chrono >= STEP{
			chrono -= STEP;

			snake.head.0 += snake.direction.0;
			snake.head.1 += snake.direction.1;
			
			snake.body.push_front(snake.head);
		}

		snake.frame = (screen_width(), screen_height()); 
		
		//gestion des touches
		if is_key_pressed(KeyCode::Right) && !fait_demitour((CELL ,0.)   ,snake.body.get(1), &snake.head) 
		{ 
			snake.direction.0 =  CELL; snake.direction.1 =  0.  ; 
		}
		if is_key_pressed(KeyCode::Down ) && !fait_demitour((0.   , CELL),snake.body.get(1), &snake.head) 
		{ 
			snake.direction.0 =  0.  ; snake.direction.1 =  CELL; 
		}
		if is_key_pressed(KeyCode::Up   ) && !fait_demitour((0.   ,-CELL),snake.body.get(1), &snake.head) 
		{ 
			snake.direction.0 =  0.  ; snake.direction.1 = -CELL; 
		}
		if is_key_pressed(KeyCode::Left ) && !fait_demitour((-CELL,0.   ),snake.body.get(1), &snake.head) 
		{ 
			snake.direction.0 = -CELL; snake.direction.1 =  0.  ; 
		}

		//verification pomme touché
		if snake.head == snake.apple {
			snake.add_point();
			snake.new_apple();
		}
		else if snake.body.len() > snake.score as usize +3 {
			snake.body.pop_back();
		}

		next_frame().await;
	}

}

// retourne vrai si la direction voulue est l'opposé de l'actuelle
fn fait_demitour(new:(f32,f32), direction:Option<&(f32, f32)>, tete:&(f32,f32) ) -> bool{
		if direction.is_none(){
			return false;
		}

		let direct = direction.unwrap();
		return direct.0 == tete.0 + new.0 && direct.1 == tete.1+ new.1;
}

