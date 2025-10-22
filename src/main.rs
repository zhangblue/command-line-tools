use clap::Parser;
use rust_cli::{Opts, SubCommand};

fn main() {
    let opts = Opts::parse();

    match opts.cmd {
        SubCommand::Base64(sub_cmd) => {
            println!("base64----{:?}", sub_cmd)
        }
    }
}
