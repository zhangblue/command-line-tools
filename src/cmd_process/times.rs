use crate::cmd_opts::times::TimeSubCommand;

pub fn process_times(sub_command: TimeSubCommand) -> anyhow::Result<()> {
    println!("{:?}", sub_command);

    Ok(())
}
