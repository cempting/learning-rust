



mod glyph;

pub struct Font {
	glyphs : glyph::Glyphs,
}

impl Font {
	pub fn new() -> Self {
		let mut glyphs = glyph::Glyphs::new();
		
		glyphs.insert(' ', glyph::Glyph::new(vec![	String::from("   "),
													String::from("   "),
													String::from("   "),
													String::from("   "),
													String::from("   "),
													String::from("   ")]));

		glyphs.insert(':', glyph::Glyph::new(vec![	String::from("   "),
													String::from("██╗"),
													String::from("╚═╝"),
													String::from("██╗"),
													String::from("╚═╝"),
													String::from("   ")]));

		glyphs.insert('0', glyph::Glyph::new(vec![	String::from(" ██████╗ "),
													String::from("██╔═████╗"),
													String::from("██║██╔██║"),
													String::from("████╔╝██║"),
													String::from("╚██████╔╝"),
													String::from(" ╚═════╝ ")]));

		glyphs.insert('1', glyph::Glyph::new(vec![	String::from(" ██╗"),
													String::from("███║"),
													String::from("╚██║"),
													String::from(" ██║"),
													String::from(" ██║"),
													String::from(" ╚═╝")]));

		glyphs.insert('2', glyph::Glyph::new(vec![	String::from("██████╗ "),
													String::from("╚════██╗"),
													String::from(" █████╔╝"),
													String::from("██╔═══╝ "),
													String::from("███████╗"),
													String::from("╚══════╝")]));
													
		glyphs.insert('3', glyph::Glyph::new(vec![	String::from("██████╗ "),
													String::from("╚════██╗"),
													String::from(" █████╔╝"),
													String::from(" ╚═══██╗"),
													String::from("██████╔╝"),
													String::from("╚═════╝ ")]));

		glyphs.insert('4', glyph::Glyph::new(vec![	String::from("██╗  ██╗"),
													String::from("██║  ██║"),
													String::from("███████║"),
													String::from("╚════██║"),
													String::from("     ██║"),
													String::from("     ╚═╝")]));

		glyphs.insert('5', glyph::Glyph::new(vec![	String::from("███████╗"),
													String::from("██╔════╝"),
													String::from("███████╗"),
													String::from("╚════██║"),
													String::from("███████║"),
													String::from("╚══════╝")]));

		glyphs.insert('6', glyph::Glyph::new(vec![	String::from(" ██████╗ "),
													String::from("██╔════╝ "),
													String::from("███████╗ "),
													String::from("██╔═══██╗"),
													String::from("╚██████╔╝"),
													String::from(" ╚═════╝ ")]));

		glyphs.insert('7', glyph::Glyph::new(vec![	String::from("███████╗"),
													String::from("╚════██║"),
													String::from("    ██╔╝"),
													String::from("   ██╔╝ "),
													String::from("   ██║  "),
													String::from("   ╚═╝  ")]));

		glyphs.insert('8', glyph::Glyph::new(vec![	String::from(" █████╗ "),
													String::from("██╔══██╗"),
													String::from("╚█████╔╝"),
													String::from("██╔══██╗"),
													String::from("╚█████╔╝"),
													String::from(" ╚════╝ ")]));

		glyphs.insert('9', glyph::Glyph::new(vec![	String::from(" █████╗ "),
													String::from("██╔══██╗"),
													String::from("╚██████║"),
													String::from(" ╚═══██║"),
													String::from(" █████╔╝"),
													String::from(" ╚════╝ ")]));

		glyphs.insert('A', glyph::Glyph::new(vec![	String::from(" █████╗ "),
													String::from("██╔══██╗"),
													String::from("███████║"),
													String::from("██╔══██║"),
													String::from("██║  ██║"),
													String::from("╚═╝  ╚═╝")]));

		glyphs.insert('B', glyph::Glyph::new(vec![	String::from("██████╗ "),
													String::from("██╔══██╗"),
													String::from("██████╔╝"),
													String::from("██╔══██╗"),
													String::from("██████╔╝"),
													String::from("╚═════╝ ")]));
													
		glyphs.insert('C', glyph::Glyph::new(vec![	String::from(" ██████╗"),
													String::from("██╔════╝"),
													String::from("██║     "),
													String::from("██║     "),
													String::from("╚██████╗"),
													String::from(" ╚═════╝")]));

		glyphs.insert('D', glyph::Glyph::new(vec![	String::from("██████╗ "),
													String::from("██╔══██╗"),
													String::from("██║  ██║"),
													String::from("██║  ██║"),
													String::from("██████╔╝"),
													String::from("╚═════╝ ")]));

		glyphs.insert('E', glyph::Glyph::new(vec![	String::from("███████╗"),
													String::from("██╔════╝"),
													String::from("██████╗ "),
													String::from("██╔═══╝ "),
													String::from("███████╗"),
													String::from("╚══════╝")]));

		glyphs.insert('F', glyph::Glyph::new(vec![	String::from("███████╗"),
													String::from("██╔════╝"),
													String::from("██████╗ "),
													String::from("██╔═══╝ "),
													String::from("██║     "),
													String::from("╚═╝     ")]));

		glyphs.insert('G', glyph::Glyph::new(vec![	String::from(" ██████╗"),
													String::from("██╔════╝"),
													String::from("██║ ███╗"),
													String::from("██║  ██║"),
													String::from("╚██████║"),
													String::from(" ╚═════╝")]));

		glyphs.insert('H', glyph::Glyph::new(vec![	String::from("██╗  ██╗"),
													String::from("██║  ██║"),
													String::from("███████║"),
													String::from("██╔══██║"),
													String::from("██║  ██║"),
													String::from("╚═╝  ╚═╝")]));

		glyphs.insert('I', glyph::Glyph::new(vec![	String::from("██╗"),
													String::from("██║"),
													String::from("██║"),
													String::from("██║"),
													String::from("██║"),
													String::from("╚═╝")]));

		glyphs.insert('J', glyph::Glyph::new(vec![	String::from("     ██╗"),
													String::from("     ██║"),
													String::from("     ██║"),
													String::from("██╗  ██║"),
													String::from("╚█████╔╝"),
													String::from(" ╚════╝ ")]));

		glyphs.insert('K', glyph::Glyph::new(vec![	String::from("██╗  ██╗"),
													String::from("██║ ██╔╝"),
													String::from("█████╔╝ "),
													String::from("██╔═██╗ "),
													String::from("██║  ██╗"),
													String::from("╚═╝  ╚═╝")]));

		glyphs.insert('L', glyph::Glyph::new(vec![	String::from("██╗     "),
													String::from("██║     "),
													String::from("██║     "),
													String::from("██║     "),
													String::from("███████╗"),
													String::from(" ╚═════╝")]));

		glyphs.insert('M', glyph::Glyph::new(vec![	String::from("██╗  ██╗"),
													String::from("███████║"),
													String::from("██║█ ██║"),
													String::from("██║╚═██║"),
													String::from("██║  ██║"),
													String::from("╚═╝  ╚═╝")]));

		glyphs.insert('N', glyph::Glyph::new(vec![	String::from("██╗  ██╗"),
													String::from("███╗ ██║"),
													String::from("██║█╗██║"),
													String::from("██║╚███║"),
													String::from("██║ ╚██║"),
													String::from("╚═╝  ╚═╝")]));

		glyphs.insert('O', glyph::Glyph::new(vec![	String::from(" █████╗ "),
													String::from("██╔══██╗"),
													String::from("██║  ██║"),
													String::from("██║  ██║"),
													String::from("╚█████╔╝"),
													String::from(" ╚════╝ ")]));

		glyphs.insert('P', glyph::Glyph::new(vec![	String::from("██████╗ "),
													String::from("██╔══██╗"),
													String::from("██████╔╝"),
													String::from("██╔═══╝ "),
													String::from("██║     "),
													String::from("╚═╝     ")]));

		glyphs.insert('Q', glyph::Glyph::new(vec![	String::from(" █████╗ "),
													String::from("██╔══██╗"),
													String::from("██║  ██║"),
													String::from("██║ ███║"),
													String::from("╚██████║"),
													String::from(" ╚═════╝")]));

		glyphs.insert('R', glyph::Glyph::new(vec![	String::from("██████╗ "),
													String::from("██╔══██╗"),
													String::from("██████╔╝"),
													String::from("██╔══██╗"),
													String::from("██║  ██║"),
													String::from("╚═╝  ╚═╝")]));

		glyphs.insert('S', glyph::Glyph::new(vec![	String::from(" ██████╗"),
													String::from("██╔════╝"),
													String::from("╚█████╗ "),
													String::from(" ╚═══██╗"),
													String::from("██████╔╝"),
													String::from("╚═════╝ ")]));

		glyphs.insert('T', glyph::Glyph::new(vec![	String::from("██████╗"),
													String::from("╚═██╔═╝"),
													String::from("  ██║  "),
													String::from("  ██║  "),
													String::from("  ██║  "),
													String::from("  ╚═╝  ")]));

		glyphs.insert('U', glyph::Glyph::new(vec![	String::from("██╗  ██╗"),
													String::from("██║  ██║"),
													String::from("██║  ██║"),
													String::from("██║  ██║"),
													String::from("╚█████╔╝"),
													String::from(" ╚════╝ ")]));

		glyphs.insert('V', glyph::Glyph::new(vec![	String::from("██╗  ██╗"),
													String::from("██║  ██║"),
													String::from("██║  ██║"),
													String::from("╚██╗██╔╝"),
													String::from(" ╚███╔╝ "),
													String::from("  ╚══╝  ")]));

		glyphs.insert('W', glyph::Glyph::new(vec![	String::from("██╗  ██╗"),
													String::from("██║  ██║"),
													String::from("██║█╗██║"),
													String::from("███████║"),
													String::from("██╔══██║"),
													String::from("╚═╝  ╚═╝")]));

		glyphs.insert('X', glyph::Glyph::new(vec![	String::from("██╗  ██╗"),
													String::from("╚██╗██╔╝"),
													String::from(" ╚███╔╝ "),
													String::from(" ██║██╗ "),
													String::from("██╔╝ ██╗"),
													String::from("╚═╝  ╚═╝")]));

		glyphs.insert('Y', glyph::Glyph::new(vec![	String::from("██╗  ██╗"),
													String::from("██║  ██║"),
													String::from("╚██╗██╔╝"),
													String::from(" ╚███╔╝ "),
													String::from("  ███║  "),
													String::from("  ╚══╝  ")]));

		glyphs.insert('Z', glyph::Glyph::new(vec![	String::from("███████╗"),
													String::from("╚═══██╔╝"),
													String::from("  ███╔╝ "),
													String::from(" ██╔═╝  "),
													String::from("███████╗"),
													String::from("╚══════╝")]));							
		Font{ glyphs }
	}

	pub fn print(&self, input : &str) {
		
		let mut line : glyph::Glyph = glyph::Glyph::new(vec![String::from(""); 6]);

		// create output lines
		for character in input.chars() {
			if let Some(glyph) = self.glyphs.get(&character) {
				line.append(glyph);
			}
		}

		line.print();
	}  
}

