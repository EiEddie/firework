// use rand::{rngs::ThreadRng, thread_rng, Rng};
use std::time::Instant;

struct Timer {
	start_instant: Instant,

	since_start_time: f64,
	fps_frame_cnt:    u128,
	prev_dalta_time:  f64,
	// rng: ThreadRng,
}

impl Timer {
	fn new() -> Self {
		Self {
			start_instant: Instant::now(),
			since_start_time: 0.,
			fps_frame_cnt: 0,
			prev_dalta_time: 0.,
			// rng: thread_rng(),
		}
	}

	/// 标记一个帧
	///
	/// # Returns
	///
	/// 返回自上次标记到这次标记之间的时间
	fn mark_frame(&mut self) -> f64 {
		let last_since_start_time = self.since_start_time;
		self.since_start_time = self.start_instant.elapsed().as_secs_f64();
		self.prev_dalta_time = self.since_start_time - last_since_start_time;
		self.fps_frame_cnt += 1;
		return self.prev_dalta_time;
	}

	fn delta(&self) -> f64 {
		return self.prev_dalta_time;
	}

	fn frame_cnt(&self) -> u128 {
		return self.fps_frame_cnt;
	}

	// /// 使用概率的计时器
	// ///
	// /// 根据给定的频率输出值
	// #[deprecated]
	// fn ticker(&mut self, freq: f64) -> bool {
	// 	let r: f64 = self.rng.gen();
	// 	return freq * self.next_dalta_time > r;
	// }
}

struct Ticker {
	start_instant:    Instant,
	since_start_time: f64,

	given_delta_time: f64,
	since_last_time:  f64,
}

impl Ticker {
	fn new(freq: f64) -> Self {
		Self { start_instant:    Instant::now(),
		       since_start_time: 0.,
		       given_delta_time: 1. / freq,
		       since_last_time:  0., }
	}

	fn ticker(&mut self) -> Option<f64> {
		let last_since_start_time = self.since_start_time;
		self.since_start_time = self.start_instant.elapsed().as_secs_f64();
		self.since_last_time += self.since_start_time - last_since_start_time;

		if self.since_last_time > self.given_delta_time {
			let tmp = self.since_last_time;
			self.since_last_time = 0.;
			return Some(tmp);
		} else {
			return None;
		}
	}
}
