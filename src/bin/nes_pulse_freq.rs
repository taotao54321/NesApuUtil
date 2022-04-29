use clap::Parser;

use nes_apu_util::pulse;

/// 周波数を NES APU 矩形波タイマー値に変換する。
#[derive(Debug, Parser)]
struct Cli {
    #[clap(required = true)]
    freqs: Vec<f64>,
}

fn main() {
    let cli = Cli::parse();

    println!("# freq\ttimer");

    for freq in cli.freqs {
        let t = pulse::freq_to_timer(freq);

        println!("{freq:.2}\t{t}");
    }
}
