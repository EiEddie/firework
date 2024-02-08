mod display;
mod error;
mod term;

use std::io;

use error::*;

fn run() -> Result<()> {
	let mut stdout = io::stdout();
	term::init(&mut stdout)?;
	display::display(&mut stdout)?;
	term::exit(&mut stdout)?;
	Ok(())
}

fn main() {
	if let Err(err) = run() {
		let mut stderr = io::stderr();
		error::error_handler(&err, &mut stderr);
	}
}
