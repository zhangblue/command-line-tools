mod cmd_opts;
mod cmd_process;
mod component;
mod error;

pub use cmd_opts::Opts;
pub use cmd_opts::SubCommand;
pub use cmd_opts::base64::Base64SubCommand;
pub use error::Error;
pub use error::Result;

pub use cmd_process::process_base64;
pub use cmd_process::process_json;
pub use cmd_process::process_times;
pub use cmd_process::process_photo;
pub use cmd_process::process_files;
