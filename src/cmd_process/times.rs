use crate::cmd_opts::times::TimeSubCommand;
use crate::error;

pub fn process_times(sub_command: TimeSubCommand) -> error::Result<()> {
    println!("{:?}", sub_command);

    Ok(())
}
