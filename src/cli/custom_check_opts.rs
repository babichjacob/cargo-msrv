use clap::Args;

#[derive(Debug, Args)]
#[command(next_help_heading = "Custom check options")]
pub struct CargoCheckOpts {
    /// Supply a custom `check` command to be used by cargo msrv
    #[arg(last = true)]
    pub custom_check_command: Option<Vec<String>>,
}
