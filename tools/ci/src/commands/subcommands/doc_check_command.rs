use crate::commands::{Flag, Prepare, PreparedCommand};
use argh::FromArgs;
use xshell::cmd;

/// Checks that all docs compile.
#[derive(FromArgs, Default)]
#[argh(subcommand, name = "doc-check")]
pub(crate) struct DocCheckCommand {}

impl Prepare for DocCheckCommand {
    fn prepare<'a>(&self, sh: &'a xshell::Shell, flags: Flag) -> Vec<PreparedCommand<'a>> {
        let quiet_flag = flags
            .contains(Flag::QUIET)
            .then_some("--quiet")
            .unwrap_or_default();

        vec![PreparedCommand::new::<Self>(
            cmd!(
                sh,
                "cargo doc --workspace --all-features --no-deps --document-private-items {quiet_flag}"
            ),
            "Please fix doc warnings in output above.",
        )]
    }
}
