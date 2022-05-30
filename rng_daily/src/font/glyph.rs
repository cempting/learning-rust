use std::collections::HashMap;

pub struct Glyph(Vec<String>);

pub type Glyphs = HashMap<char, Glyph>;

impl Glyph {
	pub fn new(map : Vec<String>) -> Self {
		Glyph(map)
	}

	pub fn print(&self) {
		for line in &self.0 {
			println!("{}", line);
		}
	}

	pub fn _get_line(&self, line_number : usize) -> &String {
		self.0.get(line_number).unwrap()
	}

	fn get(&self, row : u8) -> &String {
		let row = row as usize;
		self.0.get(row).unwrap()
	}

	fn set(&mut self, row : u8, value : String) {
		let row = row as usize;
		self.0[row] = value;
	}
	
	pub fn append(&mut self, glyph : &Glyph ) {
		for i in 0..6 {
			self.set(i, format!("{}{}", self.get(i), glyph.get(i)));
		}
	}

	pub fn _get_width(&self) -> usize {
		self.0.get(0).unwrap().chars().count()
	}

	pub fn _get_height(&self) -> usize {
		6
	}
}


