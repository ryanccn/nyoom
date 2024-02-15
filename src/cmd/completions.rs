use clap_complete::{generate, Shell};
use std::io::stdout;

use async_trait::async_trait;
use clap::{CommandFactory, Parser};
use color_eyre::eyre::Result;

#[derive(Parser)]
pub struct CompletionCommand {
    /// Shell
    #[arg(value_enum)]
    shell: Shell,
}

#[async_trait]
impl super::Command for CompletionCommand {
    async fn action(&self, _global_options: &super::Cli) -> Result<()> {
        let cmd = &mut super::Cli::command();
        generate(self.shell, cmd, cmd.get_name().to_string(), &mut stdout());

        Ok(())
    }
}
