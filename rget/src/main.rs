extern crate clap;
use indicatif::{style, ProgressBar, ProgressStyle};
use clap::{Arg, App};

fn create_progress_bar(quiet_mode: bool, msg: &str, length: Option<u64>) -> ProgressBar {
    let bar = match quiet_mode {
        true => ProgressBar::hidden(),
        false => {
            match length {
                Some(len) => ProgressBar::new(len),
                None => ProgressBar::new_spinner()
            }
        }
    };

    let style_result = ProgressStyle::default_bar()
        .template("{msg} {spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} eta: {eta}");

    match style_result {
        Ok(style) => {
            let final_style = style.progress_chars("=> ");
            bar.set_style(final_style);
        },
        Err(e) => eprintln!("Template error: {}", e),
    };

    bar.set_message(msg.to_string());

    bar
}


fn main() {
    let matches = App::new("Rget")
        .arg(Arg::with_name("URL")
                .required(true)
                .takes_value(true)
                .index(1)
                .help("url to download"))
        .get_matches();

    let url = matches.value_of("URL").unwrap();
    print!("{}", url);
}
