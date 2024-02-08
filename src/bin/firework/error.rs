use std::io;

use crossterm::style::Stylize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	#[error("{0}")]
	Msg(String),

	#[error(transparent)]
	IoError(#[from] ::std::io::Error),
}

impl From<&'static str> for Error {
	fn from(s: &'static str) -> Self {
		Error::Msg(s.to_owned())
	}
}

impl From<String> for Error {
	fn from(s: String) -> Self {
		Error::Msg(s)
	}
}

pub type Result<T> = ::std::result::Result<T, Error>;

pub fn error_handler(err: &Error, out: &mut impl io::Write) {
	if let Error::IoError(io_err) = err {
		if io_err.kind() == io::ErrorKind::BrokenPipe {
			::std::process::exit(0);
		}
	}

	writeln!(out, "{}: {}", "[error]".red(), err).ok();
}
