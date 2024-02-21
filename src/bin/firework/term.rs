use std::io;

use colorsys::Rgb;
use crossterm::style::{self, Stylize};
use crossterm::{cursor, execute, queue, terminal};
use firework::arg::BACKGROUND_COLOR;
use firework::CanvasSize;

use crate::error::*;

pub struct CSize {}

impl CanvasSize for CSize {
	fn width(&self) -> u32 {
		terminal::size().expect("Unable to get terminal size").0 as u32
	}

	fn height(&self) -> u32 {
		terminal::size().expect("Unable to get terminal size").1 as u32
	}
}

pub static SIZE: CSize = CSize {};

fn set_background_color(out: &mut impl io::Write, color: &Rgb) -> Result<()> {
	let color = style::Color::Rgb { r: color.red() as u8,
	                                g: color.green() as u8,
	                                b: color.blue() as u8, };
	let mask = " ".repeat(SIZE.width() as usize).on(color);

	for r in 0..SIZE.height() as u16 {
		queue!(out, cursor::MoveTo(0, r), style::Print(&mask))?;
	}

	out.flush()?;
	Ok(())
}

/// 初始化标准输出, 进入显示模式
///
/// - 启用终端原始模式:
///   屏蔽键盘输出与控制字符
/// - 切换到备用屏幕
/// - 隐藏光标
/// - 清除终端
/// - 设置终端背景颜色为黑色
pub fn init(out: &mut impl io::Write) -> Result<()> {
	terminal::enable_raw_mode()?;
	execute!(
	         out,
	         terminal::EnterAlternateScreen,
	         cursor::Hide,
	         terminal::Clear(terminal::ClearType::All),
	)?;
	set_background_color(out, &Rgb::from(BACKGROUND_COLOR))?;
	Ok(())
}

/// 退出显示模式
///
/// - 重置终端背景颜色
/// - 显示光标
/// - 退出备用屏幕
/// - 禁用终端原始模式
pub fn exit(out: &mut impl io::Write) -> Result<()> {
	execute!(
	         out,
	         style::ResetColor,
	         cursor::Show,
	         terminal::LeaveAlternateScreen,
	)?;
	terminal::disable_raw_mode()?;
	Ok(())
}
