use clap::Parser;
use rust_cmd::{
    Opts, SubCommand, process_base64, process_files, process_json, process_photo,
    process_port_scanner, process_times,
};

#[tokio::main]
async fn main() {
    let opts = Opts::parse();

    let result = match opts.cmd {
        SubCommand::Base64(sub_command) => process_base64(sub_command),
        SubCommand::Date(sub_command) => process_times(sub_command),
        SubCommand::Json(opts) => process_json(opts),
        SubCommand::Photo(sub_command) => process_photo(sub_command),
        SubCommand::Files(sub_command) => process_files(sub_command),
        SubCommand::PortScanner(opts) => process_port_scanner(opts).await,
    };

    if let Err(err) = result {
        err.print_error_msg();
    }
}
