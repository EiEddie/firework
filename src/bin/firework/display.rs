use std::io;
use std::time::Duration;

use crossterm::event::*;
use crossterm::style::Stylize;
use crossterm::*;
use firework::*;

use crate::error::*;
use crate::term::SIZE;

pub fn display(out: &mut impl io::Write) -> Result<()> {
	let mut timer = time::Timer::new();

	// 控制火箭升空的计时器
	let mut launch_ticker = time::Ticker::new(arg::BIG_ROCKET_LAUNCH_FREQ);
	// 控制烟花产生粒子的计时器
	let mut glitter_ticker = time::Ticker::new(arg::GLITTER_GENERATE_FREQ);

	// 烟花粒子及所有火箭的管理器
	let mut glitters = rocket::Glitters::new();

	loop {
		// 捕捉事件, 退出循环
		if event::poll(Duration::from_secs_f64(1. / arg::FPS))? {
			if let Event::Key(KeyEvent { code, .. }) = event::read()? {
				match code {
					// 输入 `ESC`, `q`, `e` 退出
					KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('e') => {
						break;
					},
					_ => {},
				}
			}
		}

		let dt = timer.mark_frame();

		// 每隔一段时间就发射一枚火箭
		if let Some(_) = launch_ticker.ticker() {
			glitters.launch(&SIZE);
		}

		// 更新火箭与粒子的状态
		glitters.update_rockets(dt);
		glitters.update_glitters(dt);

		if let Some(_) = glitter_ticker.ticker() {
			// 每隔一段时间就生成更多粒子
			glitters.gen_glitters(&SIZE);
		}

		// 将粒子显示出来
		// 打印粒子到屏幕
		for (pos, color) in glitters.iter() {
			let style = style::Color::Rgb { r: color.red() as u8,
			                                g: color.green() as u8,
			                                b: color.blue() as u8, };
			queue!(
			       out,
			       cursor::MoveTo(pos.0 as u16, pos.1 as u16),
			       style::Print(arg::DISPLAY_CHAR.with(style))
			)?;
		}

		// 调试信息
		#[cfg(debug_assertions)]
		{
			queue!(
			       out,
			       cursor::MoveTo(0, 0),
			       style::Print(format!(
				// fps; brc: big rocket count
				// src: small rocket count; ptc: particle count
				// w: width; h: height
				"fps:{:.3} brc:{:<4} src:{:<4} ptc:{:<4}; w:{} h:{}",
				1. / dt,
				glitters.cnt_big_rocket(),
				glitters.cnt_small_rocket(),
				glitters.cnt_particle(),
				SIZE.width(),
				SIZE.height()
			))
			)?;
		}

		// 刷新屏幕
		out.flush()?;
	}

	Ok(())
}
