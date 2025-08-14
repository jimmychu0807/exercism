use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
	hours: u32,
	minutes: u32,
}

impl Clock {
	pub fn new(hours: i32, minutes: i32) -> Self {
		let (clock_hr, clock_min) = Self::add_time((0, 0), (hours, minutes));
		Clock { hours: clock_hr, minutes: clock_min }
	}

	pub fn add_minutes(&self, minutes: i32) -> Self {
		Clock::new(self.hours as i32, self.minutes as i32 + minutes)
	}

	// -- helper methods --
	fn add_time(base: (u32, u32), offset: (i32, i32)) -> (u32, u32) {
		let mut clock_min = base.1 as i32 + offset.1;
		let mut hr_decrement = 0;
		while clock_min < 0 {
			clock_min += 60;
			hr_decrement += 1;
		}

		let mut clock_hr = base.0 as i32 - hr_decrement + offset.0;
		while clock_hr < 0 {
			clock_hr += 24;
		}

		clock_hr = (clock_hr + clock_min / 60) % 24;
		clock_min %= 60;
		(clock_hr as u32, clock_min as u32)
	}
}

impl fmt::Display for Clock {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:02}:{:02}", self.hours, self.minutes)
	}
}
