use std::collections::VecDeque;
use macroquad::rand::gen_range;

pub struct Snake{
	pub direction : (f32, f32),
	pub body :  VecDeque<(f32, f32)>,
	pub head : (f32, f32),
	pub apple: (f32, f32),
	pub score:  i64,
	pub frame: (f32,f32),
}

pub const CELL: f32 = 25.;

impl Snake{
	pub fn new() -> Self {
		Snake {
			direction : (0., 25.),
			body      : VecDeque::new(),
			head      : (0., 0.),
			apple     : (0., 0.),
			score     :  0,
			frame     : (0., 0.)
		}
	}

	pub fn add_point(&mut self){ self.score += 1; }

	pub fn new_snake(&mut self){
		self.head =         ((self.frame.0 as i32 /50*25) as f32,CELL*3.);
		self.body.push_front(self.head);

		self.body.push_back (((self.frame.0 as i32 /50*25) as f32,CELL*2.));
		self.body.push_back (((self.frame.0 as i32 /50*25) as f32,CELL   ));
	}

	pub fn supprimer_cell(&mut self){
		self.body.pop_back();
	}

	pub fn new_apple(&mut self){
        loop{
            self.apple.0 = ((gen_range(0 , self.frame.0 as i32)/25)*25) as f32 ;
		    self.apple.1 = ((gen_range(25, self.frame.1 as i32)/25)*25) as f32 ;
            let mut pomme_sur_serpent = false;
            for cell in &self.body {
                if cell == &self.apple { pomme_sur_serpent = true;}
            }
            if !pomme_sur_serpent {break;}
        }
		
	}
}