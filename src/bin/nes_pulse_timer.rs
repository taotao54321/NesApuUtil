use clap::Parser;

use nes_apu_util::pulse;

/// NES APU 矩形波タイマー値を周波数に変換する。
#[derive(Debug, Parser)]
struct Cli {
    #[clap(required = true)]
    timers: Vec<u16>,
}

fn main() {
    let cli = Cli::parse();

    println!("# timer\tfreq");

    for t in cli.timers {
        let freq = pulse::timer_to_freq(t);

        println!("{t}\t{freq:.2}");
    }
}
