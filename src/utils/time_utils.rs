use crate::TICK_RATE;

pub fn ticks_to_seconds(ticks: u64) -> f64{
	return ticks as f64 / TICK_RATE as f64;
}
