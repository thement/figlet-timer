use figlet_rs::FIGfont;
use clap::Parser;
use std::time::{Instant, Duration};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    minutes: u64,
}

fn draw_message(message: &str, font: &FIGfont) {
    print!("\x1b[1;1H");
    let figure = font.convert(message).expect("BUG: cannot format font");
    for s in figure.to_string().split("\n") {
        println!("{}\x1b[K", s);
    }
}

fn main() {
    let cli = Cli::parse();
    //let font_data = std::include_str!("univers.flf");
    //let font = FIGfont::from_content(font_data).expect("BUG: cannot create font");
    let font = FIGfont::standand().expect("BUG: missing standard font");
    let timer_seconds = cli.minutes * 60;
    let timer_duration = Duration::from_secs(timer_seconds);
    let end_at = Instant::now()
        .checked_add(timer_duration)
        .expect("BUG: time in too distant future");
    let mut last_seconds = None;

    println!("\x1b[2J");
    loop {
        match end_at.checked_duration_since(Instant::now()) {
            None => break,
            Some(duration) => {
                let new_seconds = Some(duration.as_secs());
                if new_seconds != last_seconds {
                    let seconds = duration.as_secs() + 1;
                    let message = format!("{}:{:02}", seconds / 60, seconds % 60);
                    draw_message(&message, &font);
                    last_seconds = new_seconds;
                }
            }
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    draw_message("ding!", &font);
}
