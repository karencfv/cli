//! The Oxide command line tool.
#![deny(missing_docs)]

use clap::{App, IntoApp, Parser};
use clap_complete::{generate, Shell};

use std::io;

/// Work seamlessly with Oxide from the command line.
#[derive(Parser, Debug, Clone)]
#[clap(version = clap::crate_version!(), author = clap::crate_authors!("\n"))]
struct Opts {
    /// Print debug info
    #[clap(short, long)]
    debug: bool,

    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser, Debug, Clone)]
enum SubCommand {
    Completion(CmdCompletion),
}

/// Generate shell completion scripts.
///
/// When installing Oxide CLI through a package manager, it's possible that
/// no additional shell configuration is necessary to gain completion support. For
/// Homebrew, see <https://docs.brew.sh/Shell-Completion>.
///
/// If you need to set up completions manually, follow the instructions below. The exact
/// config file locations might vary based on your system. Make sure to restart your
/// shell before testing whether completions are working.
///
/// ### bash
///
/// First, ensure that you install `bash-completion` using your package manager.
///
/// After, add this to your `~/.bash_profile`:
///
///         eval "$(oxide completion -s bash)"
///
/// ### zsh
/// Generate a `_oxide` completion script and put it somewhere in your `$fpath`:
///
///         oxide completion -s zsh > /usr/local/share/zsh/site-functions/_oxide
///
/// Ensure that the following is present in your `~/.zshrc`:
///         autoload -U compinit
///         compinit -i
///
/// Zsh version 5.7 or later is recommended.
///
/// ### fish
///
/// Generate a `oxide.fish` completion script:
///
///         oxide completion -s fish > ~/.config/fish/completions/oxide.fish
///
/// ### PowerShell
///
/// Open your profile script with:
///
///         mkdir -Path (Split-Path -Parent $profile) -ErrorAction SilentlyContinue
///         notepad $profile
///
/// Add the line and save the file:
///
/// Invoke-Expression -Command $(oxide completion -s powershell | Out-String)
#[derive(Parser, Debug, Clone)]
#[clap(verbatim_doc_comment)]
pub struct CmdCompletion {
    #[clap(short, long)]
    shell: Shell,
}

fn main() {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Completion(cmd) => {
            // Convert our opts into a clap app.
            let mut app: App = Opts::into_app();
            let name = app.get_name().to_string();
            // Generate the completion script.
            generate(cmd.shell, &mut app, name, &mut io::stdout());
        }
    }
}
