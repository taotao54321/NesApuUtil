//! NES APU に関するライブラリ。NTSC のみ想定している。

// ref: https://www.nesdev.org/wiki/Cycle_reference_chart
const CPU_FREQ: f64 = 236250000.0 / (11.0 * 12.0);

pub mod pulse;
pub mod triangle;
