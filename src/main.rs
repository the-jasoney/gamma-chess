use clap::{arg, command};

pub mod chess;
pub mod piece;
pub mod ui;

fn main() {
    let matches = command!()
        .arg(
            arg!(--uci)
                .action(clap::ArgAction::SetTrue)
                .help("Launch with no GUI, interacting solely via UCI."),
        )
        .get_matches();

    if matches.get_flag("uci") {
        todo!()
    } else {
        todo!()
    }
}
