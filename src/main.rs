use macroquad::color::{GREEN, WHITE, RED};

use macroquad::input::{KeyCode, is_key_pressed};
use macroquad::shapes::draw_rectangle;
use macroquad::text::draw_text;
use macroquad::ui::{hash, root_ui,widgets};
use macroquad::time::get_frame_time;
use macroquad::window::{clear_background, next_frame, screen_height, screen_width};
use std::collections::VecDeque;
use macroquad::prelude::*;
use macroquad::rand::gen_range;

struct Snake{
	direction : (f32, f32),
	body :  VecDeque<(f32, f32)>,
	head : (f32, f32),
	apple: (f32, f32),
	score:  i32,
	frame: (f32,f32),
}

const CELL: f32 = 25.;
const STEP: f32 = 0.25;

impl Snake{
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

	fn add_point(&mut self){ self.score += 1; }

	fn new_snake(&mut self){
		self.head =         ((self.frame.0 as i32 /50*25) as f32,CELL*3.);
		self.body.push_front(self.head);

		self.body.push_back (((self.frame.0 as i32 /50*25) as f32,CELL*2.));
		self.body.push_back (((self.frame.0 as i32 /50*25) as f32,CELL   ));
	}

	fn supprimer_cell(&mut self){
		self.body.pop_back();
	}

	fn new_apple(&mut self){
		self.apple.0 = ((gen_range(0 , self.frame.0 as i32)/25)*25) as f32 ;
		self.apple.1 = ((gen_range(25, self.frame.1 as i32)/25)*25) as f32 ;
	}
}

#[macroquad::main("snake")]
async fn main(){
	let snake = Snake::new();
	main_menu(snake).await;
}

async fn main_menu( mut snake: Snake){

	let mut play_clicked = false;

	widgets::Button::new("Play").position(vec2(65.0, 15.0)) ;
	loop{
		clear_background(WHITE);
		root_ui().window(hash!(), vec2(screen_width( ) / 2. - 100., 150.), vec2(200., 250.), |ui| {
			ui.label(None, "Snake");
			if ui.button(None, "Play"){ play_clicked = true; }
			if ui.button(None, "Exit"){ std::process::exit(0)      ;}
		});

		if play_clicked {
			game_loop(&mut snake).await;
		}

		next_frame().await;
	}
}

async fn game_loop(snake_in: &mut Snake){
	let mut snake = snake_in;
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

			collision   (&snake.head, &snake.body);
			pomme_touche(&mut snake);
			sortie      (&snake.frame, &snake.head);
			snake.body.push_front(snake.head);
		}

		snake.frame = (screen_width(), screen_height()); 

		//gestion des touches
		if is_key_pressed(KeyCode::Right) && !fait_demitour((CELL ,0.   ),snake.body.get(1), &snake.head) 
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

		next_frame().await;
	}
}

// retourne vrai si la direction voulue est l'opposé de l'actuelle
fn fait_demitour (new_pos:(f32,f32), direction:Option<&(f32, f32)>, head:&(f32,f32) ) -> bool{
		if direction.is_none(){
			return false;
		}

		let direct = direction.unwrap();
		return direct.0 == head.0 + new_pos.0 && direct.1 == head.1+ new_pos.1;
}

fn pomme_touche (snake: &mut Snake){
	if snake.head == snake.apple {
		snake.add_point();
		snake.new_apple();
	}
	else if snake.body.len() > snake.score as usize +3 {
		snake.supprimer_cell();
	}
}

fn collision (head:&(f32,f32), body:&VecDeque<(f32, f32)>) -> bool{
	for cell in body {
		if cell == head {
			return true;
		}
	}
	return false;
}

fn sortie (frame:&(f32,f32), body:&(f32,f32)) -> bool {
	return body.0 < 0. || body.1 < 0. || body.0 > frame.0 || body.1 > frame.1 ;
}

