use std::{borrow::Cow, fmt::Display};

pub const RESET: &str = "\x1b[0m";
pub const BOLD: &str = "\x1b[1m";

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Color {
	Black,
	Red,
	Green,
	Yellow,
	Blue,
	Magenta,
	Cyan,
	White,
	Default,
	Reset,
	Indexed(u8),
}

impl Color {
	pub fn fg_code(&self) -> Cow<'static, str> {
		match self {
			Color::Black => Cow::Borrowed("30"),
			Color::Red => Cow::Borrowed("31"),
			Color::Green => Cow::Borrowed("32"),
			Color::Yellow => Cow::Borrowed("33"),
			Color::Blue => Cow::Borrowed("34"),
			Color::Magenta => Cow::Borrowed("35"),
			Color::Cyan => Cow::Borrowed("36"),
			Color::White => Cow::Borrowed("37"),
			Color::Default => Cow::Borrowed("39"),
			Color::Reset => Cow::Borrowed("0"),
			Color::Indexed(index) => Cow::Owned(index.to_string()),
		}
	}

	pub fn bg_code(&self) -> Cow<'static, str> {
		match self {
			Color::Black => Cow::Borrowed("40"),
			Color::Red => Cow::Borrowed("41"),
			Color::Green => Cow::Borrowed("42"),
			Color::Yellow => Cow::Borrowed("43"),
			Color::Blue => Cow::Borrowed("44"),
			Color::Magenta => Cow::Borrowed("45"),
			Color::Cyan => Cow::Borrowed("46"),
			Color::White => Cow::Borrowed("47"),
			Color::Default => Cow::Borrowed("49"),
			Color::Reset => Cow::Borrowed("0"),
			Color::Indexed(index) => Cow::Owned(index.to_string()),
		}
	}
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct AnsiBuilder {
	fg: Option<Color>,
	bg: Option<Color>,
	bold: bool,
	dim: bool,
	italics: bool,
}

impl AnsiBuilder {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn fg(&mut self, color: Color) -> &mut Self {
		self.fg = Some(color);
		self
	}

	pub fn bg(&mut self, color: Color) -> &mut Self {
		self.bg = Some(color);
		self
	}

	pub fn bold(&mut self) -> &mut Self {
		self.bold = true;
		self
	}

	pub fn dim(&mut self) -> &mut Self {
		self.dim = true;
		self
	}

	pub fn italics(&mut self) -> &mut Self {
		self.italics = true;
		self
	}

	pub fn finish(&self) -> String {
		let mut operators = Vec::<&'static str>::with_capacity(5);
		let fg_code = self.fg.map(|fg| fg.fg_code());
		let bg_code = self.bg.map(|bg| bg.bg_code());
		if let Some(code) = &fg_code {
			operators.push(code.as_ref());
		}
		if let Some(code) = &bg_code {
			operators.push(code.as_ref());
		}
		if self.bold {
			operators.push("1");
		}
		if self.dim {
			operators.push("2");
		}
		if self.italics {
			operators.push("3");
		}
		format!("\x1b[{}m", operators.join(";"))
	}
}

impl Display for AnsiBuilder {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.finish())
	}
}

#[inline]
pub fn ansi() -> AnsiBuilder {
	AnsiBuilder::default()
}

#[inline]
pub fn fg(color: Color) -> String {
	ansi().fg(color).finish()
}

#[inline]
pub fn bg(color: Color) -> String {
	ansi().bg(color).finish()
}
