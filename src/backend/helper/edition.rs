use super::writer::Writer;
use std::{fmt::Display, io::Result};

pub struct Infix<T> {
	rhs: &'static str,
	inner: T,
}

impl<T> Display for Infix<T>
where
	T: Display,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.inner.fmt(f)?;
		self.rhs.fmt(f)
	}
}

pub trait Edition {
	fn runtime(&self) -> &'static str;

	fn start_block(&self, w: Writer) -> Result<()>;
	fn start_loop(&self, level: usize, w: Writer) -> Result<()>;
	fn start_if(&self, cond: &str, w: Writer) -> Result<()>;
	fn end_block(&self, level: usize, w: Writer) -> Result<()>;
	fn end_loop(&self, w: Writer) -> Result<()>;
	fn end_if(&self, level: usize, w: Writer) -> Result<()>;

	fn br_target(&self, level: usize, in_loop: bool, w: Writer) -> Result<()>;
	fn br_to_level(&self, level: usize, up: usize, is_loop: bool, w: Writer) -> Result<()>;

	fn i64(&self, i: i64) -> Infix<i64>;
}

pub struct LuaJIT;

impl Edition for LuaJIT {
	fn runtime(&self) -> &'static str {
		"luajit"
	}

	fn start_block(&self, w: Writer) -> Result<()> {
		writeln!(w, "do")
	}

	fn start_loop(&self, level: usize, w: Writer) -> Result<()> {
		writeln!(w, "do")?;
		writeln!(w, "::continue_at_{}::", level)
	}

	fn start_if(&self, cond: &str, w: Writer) -> Result<()> {
		writeln!(w, "if {} ~= 0 then", cond)
	}

	fn end_block(&self, level: usize, w: Writer) -> Result<()> {
		writeln!(w, "::continue_at_{}::", level)?;
		writeln!(w, "end")
	}

	fn end_loop(&self, w: Writer) -> Result<()> {
		writeln!(w, "end")
	}

	fn end_if(&self, level: usize, w: Writer) -> Result<()> {
		writeln!(w, "::continue_at_{}::", level)?;
		writeln!(w, "end")
	}

	fn br_target(&self, _level: usize, _in_loop: bool, _w: Writer) -> Result<()> {
		Ok(())
	}

	fn br_to_level(&self, level: usize, up: usize, _is_loop: bool, w: Writer) -> Result<()> {
		writeln!(w, "goto continue_at_{}", level - up)
	}

	fn i64(&self, i: i64) -> Infix<i64> {
		Infix {
			rhs: "LL",
			inner: i,
		}
	}
}

pub struct Luau;

impl Edition for Luau {
	fn runtime(&self) -> &'static str {
		"luau"
	}

	fn start_block(&self, w: Writer) -> Result<()> {
		writeln!(w, "while true do")
	}

	fn start_loop(&self, _level: usize, w: Writer) -> Result<()> {
		writeln!(w, "while true do")
	}

	fn start_if(&self, cond: &str, w: Writer) -> Result<()> {
		writeln!(w, "while true do")?;
		writeln!(w, "if {} ~= 0 then", cond)
	}

	fn end_block(&self, _level: usize, w: Writer) -> Result<()> {
		writeln!(w, "break")?;
		writeln!(w, "end")
	}

	fn end_loop(&self, w: Writer) -> Result<()> {
		writeln!(w, "break")?;
		writeln!(w, "end")
	}

	fn end_if(&self, _level: usize, w: Writer) -> Result<()> {
		writeln!(w, "end")?;
		writeln!(w, "break")?;
		writeln!(w, "end")
	}

	fn br_target(&self, level: usize, in_loop: bool, w: Writer) -> Result<()> {
		writeln!(w, "if desired then")?;
		writeln!(w, "if desired == {} then", level)?;
		writeln!(w, "desired = nil")?;

		if in_loop {
			writeln!(w, "continue")?;
		}

		writeln!(w, "end")?;
		writeln!(w, "break")?;
		writeln!(w, "end")
	}

	fn br_to_level(&self, level: usize, up: usize, is_loop: bool, w: Writer) -> Result<()> {
		writeln!(w, "do")?;

		if up == 0 {
			if is_loop {
				writeln!(w, "continue")?;
			} else {
				writeln!(w, "break")?;
			}
		} else {
			writeln!(w, "desired = {}", level - up)?;
			writeln!(w, "break")?;
		}

		writeln!(w, "end")
	}

	fn i64(&self, i: i64) -> Infix<i64> {
		Infix { rhs: "", inner: i }
	}
}
