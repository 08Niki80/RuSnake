use macroquad::color::{GREEN, WHITE, RED};
use macroquad::input::{KeyCode, is_key_pressed};
use macroquad::shapes::draw_rectangle;
use macroquad::text::draw_text;
use macroquad::ui::{hash, root_ui};
use macroquad::time::get_frame_time;
use macroquad::window::{clear_background, next_frame, screen_height, screen_width};
use macroquad::prelude::*;

mod verifs;
mod snake;

const CELL: f32 = 25.;
const STEP: f32 = 0.25;


#[macroquad::main("snake")]
async fn main(){
	main_menu().await;
}

async fn main_menu(){
	let mut tmp_score:i64 ;
	let mut max_score:i64 = 0;


	let mut play_clicked = false;
	loop{
		    
		clear_background(WHITE);
		root_ui().window(hash!(), vec2(0., 0.), vec2(screen_width(), screen_height()), |ui| {
			if max_score > 0 { ui.label(vec2(screen_width()/2. - 15., 125.), &format!("max score {}", max_score).to_string());}
			ui.label(vec2(screen_width()/2. - 15., 150.),     "Snake");
			if ui.button(vec2(screen_width()/2. - 12., 175.), "Play"){ play_clicked = true; }
			if ui.button(vec2(screen_width()/2. - 12., 200.), "Exit"){ std::process::exit(0)      ;}
		});

		if play_clicked {
			tmp_score = game_loop().await;
			if tmp_score > max_score { max_score = tmp_score; }
			play_clicked = false;
		}

		next_frame().await;
	}
}


async fn game_loop() -> i64{

	let mut serpent = snake::Snake::new();
	let mut chrono = 0. ;

	serpent.frame = (screen_width(), screen_height()); 

	serpent.new_apple();
	serpent.new_snake();

	loop{
		//nouvelle image
		clear_background(WHITE);

		draw_text     (serpent.score.to_string(),50., 50., 10.0, GREEN);

		draw_rectangle(serpent.apple.0 , serpent.apple.1, CELL, CELL, RED  );
		for block in &serpent.body{
			draw_rectangle(block.0  , block.1 , CELL, CELL, GREEN);
		}

		//avancement du serpent
		chrono += get_frame_time();

		if chrono >= STEP{
			chrono -= STEP;

			serpent.head.0 += serpent.direction.0;
			serpent.head.1 += serpent.direction.1;

			if verifs::pomme_touche(&mut serpent.head, &mut serpent.apple) {
				serpent.add_point();
				serpent.new_apple();
			}
			else if serpent.body.len() > serpent.score as usize +3 {
				serpent.supprimer_cell();
			}

			if 	verifs::collision   (&serpent.head, &serpent.body) | 
				verifs::sortie      (&serpent.frame, &serpent.head){
				
				return serpent.score;
				
			}

				

			serpent.body.push_front(serpent.head);
		}

		serpent.frame = (screen_width(), screen_height()); 

		//gestion des touches
		if is_key_pressed(KeyCode::Right) && !verifs::fait_demitour((CELL ,0.   ),serpent.body.get(1), &serpent.head) 
		{ 
			serpent.direction.0 =  CELL; serpent.direction.1 =  0.  ; 
		}
		if is_key_pressed(KeyCode::Down ) && !verifs::fait_demitour((0.   , CELL),serpent.body.get(1), &serpent.head) 
		{ 
			serpent.direction.0 =  0.  ; serpent.direction.1 =  CELL; 
		}
		if is_key_pressed(KeyCode::Up   ) && !verifs::fait_demitour((0.   ,-CELL),serpent.body.get(1), &serpent.head) 
		{ 
			serpent.direction.0 =  0.  ; serpent.direction.1 = -CELL; 
		}
		if is_key_pressed(KeyCode::Left ) && !verifs::fait_demitour((-CELL,0.   ),serpent.body.get(1), &serpent.head) 
		{ 
			serpent.direction.0 = -CELL; serpent.direction.1 =  0.  ; 
		}

		next_frame().await;
	}
}



