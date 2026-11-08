use std::collections::VecDeque;

// retourne vrai si la direction voulue est l'opposé de l'actuelle
pub fn fait_demitour (new_pos:(f32,f32), direction:Option<&(f32, f32)>, head:&(f32,f32) ) -> bool{
		if direction.is_none(){
			return false;
		}

		let direct = direction.unwrap();
		return direct.0 == head.0 + new_pos.0 && direct.1 == head.1+ new_pos.1;
}

pub fn pomme_touche (head: &mut (f32, f32) , apple: &mut (f32, f32)) -> bool{
	if head == apple {
		return true;
	}
	return false;
}

pub fn collision (head:&(f32,f32), body:&VecDeque<(f32, f32)>) -> bool{
	for cell in body {
		if cell == head {
			return true;
		}
	}
	return false;
}

pub fn sortie (frame:&(f32,f32), body:&(f32,f32)) -> bool {
	return body.0 < 0. || body.1 < 0. || body.0 > frame.0 || body.1 > frame.1 ;
}