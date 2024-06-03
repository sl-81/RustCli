extern crate clap;
use clap::{Arg, App};

// fn create_progress_bar(quiet_mode: bool, msg: &str, length: Option<u64>) -> ProgressBar {
//     let bar = match quiet_mode {
//         true => ProgressBar::hidden()
//     }
// }

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
