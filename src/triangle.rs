use crate::CPU_FREQ;

/// 三角波タイマー値を周波数に変換する。
pub fn timer_to_freq(t: u16) -> f64 {
    CPU_FREQ / (32.0 * (f64::from(t) + 1.0))
}

/// 周波数を最も近い三角波タイマー値に丸める。
///
/// `freq` は正でなければならない。
pub fn freq_to_timer(freq: f64) -> u16 {
    assert!(freq > 0.0);

    (CPU_FREQ / (32.0 * freq) - 1.0) as u16
}
