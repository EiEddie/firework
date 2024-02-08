use std::io;
use std::time::Duration;

use crossterm::event::*;
use crossterm::style::Stylize;
use crossterm::*;
use firework::*;

struct CSize {}

impl CanvasSize for CSize {
	fn width(&self) -> u32 {
		terminal::size().unwrap().0 as u32
	}

	fn height(&self) -> u32 {
		terminal::size().unwrap().1 as u32
	}
}

static SIZE: CSize = CSize {};

fn mainloop(out: &mut impl io::Write) {
	let mut timer = time::Timer::new();

	// 控制火箭升空的计时器
	let mut launch_ticker = time::Ticker::new(arg::BIG_ROCKET_LAUNCH_FREQ);
	// 控制烟花产生粒子的计时器
	let mut glitter_ticker = time::Ticker::new(arg::GLITTER_GENERATE_FREQ);

	// 烟花粒子及所有火箭的管理器
	let mut glitters = rocket::Glitters::new();

	loop {
		if event::poll(Duration::from_secs_f64(1. / arg::FPS)).unwrap() {
			if let Event::Key(KeyEvent { code, .. }) = event::read().unwrap() {
				match code {
					KeyCode::Esc => {
						break;
					},
					_ => {},
				}
			}
		}

		let dt = timer.mark_frame();

		if let Some(_) = launch_ticker.ticker() {
			glitters.launch(&SIZE);
		}

		glitters.update_rockets(dt);
		glitters.update_glitters(dt);

		if let Some(_) = glitter_ticker.ticker() {
			glitters.gen_glitters(&SIZE);
		}

		let _ = out.queue(terminal::Clear(terminal::ClearType::Purge));
		for (pos, color) in glitters.iter() {
			let _ = queue!(
			               out,
			               cursor::MoveTo(pos.0 as u16, pos.1 as u16),
			               style::Print(arg::DISPLAY_CHAR.with(
				style::Color::Rgb { r: color.red() as u8,
				                    g: color.green() as u8,
				                    b: color.blue() as u8, }
			))
			);
		}

		#[cfg(debug_assertions)]
		{
			let _ = queue!(
			               out,
			               cursor::MoveTo(0, 0),
			               style::Print(format!(
				// fps; brc: big rocket count
				// src: small rocket count; ptc: particle count
				"fps:{:.3} brc:{:<4} src:{:<4} ptc:{:<4}",
				1. / dt,
				glitters.cnt_big_rocket(),
				glitters.cnt_small_rocket(),
				glitters.cnt_particle()
			))
			);
		}

		let _ = out.flush();
	}
}

fn main() {
	let mut stdout = io::stdout();
	let _ = terminal::enable_raw_mode();
	let _ = execute!(
	                 &mut stdout,
	                 terminal::EnterAlternateScreen,
	                 cursor::Hide,
	                 terminal::Clear(terminal::ClearType::All)
	);
	mainloop(&mut stdout);
	let _ = execute!(
	                 &mut stdout,
	                 style::ResetColor,
	                 cursor::Show,
	                 terminal::LeaveAlternateScreen
	);
	let _ = terminal::disable_raw_mode();
}
