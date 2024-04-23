use crate::commands::{Flag, Prepare, PreparedCommand};
use argh::FromArgs;
use xshell::cmd;

/// Checks that the project compiles.
#[derive(FromArgs, Default)]
#[argh(subcommand, name = "compile-check")]
pub(crate) struct CompileCheckCommand {}

impl Prepare for CompileCheckCommand {
    fn prepare<'a>(&self, sh: &'a xshell::Shell, flags: Flag) -> Vec<PreparedCommand<'a>> {
        let quiet_flag = flags
            .contains(Flag::QUIET)
            .then_some("--quiet")
            .unwrap_or_default();

        vec![PreparedCommand::new::<Self>(
            cmd!(sh, "cargo check --workspace {quiet_flag}"),
            "Please fix compiler errors in output above.",
        )]
    }
}
