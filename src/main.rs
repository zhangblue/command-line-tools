use clap::Parser;
use rust_cmd::{Opts, SubCommand, process_base64, process_json, process_times};

fn main() {
    let opts = Opts::parse();

    let result = match opts.cmd {
        SubCommand::Base64(sub_command) => process_base64(sub_command),
        SubCommand::Time(sub_command) => process_times(sub_command),
        SubCommand::Json(opts) => process_json(opts),
    };

    if let Err(err) = result {
        err.print_error_msg();
    }
}
