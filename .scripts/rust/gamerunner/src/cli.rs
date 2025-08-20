use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
pub(crate) enum ProtonRunnable {
    Command {
        command_path: PathBuf,
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
    ExeFile {
        exe: PathBuf,
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
}

#[derive(Debug, Subcommand)]
pub(crate) enum Run {
    Mount,
    Native {
        path: PathBuf,
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
    Proton {
        #[arg(
            long,
            default_value = "/usr/share/steam/compatibilitytools.d/proton-ge-custom/proton"
        )]
        proton_path: PathBuf,
        #[command(subcommand)]
        runnable: ProtonRunnable,
    },
}

#[derive(Debug, clap::Args)]
pub(crate) struct NonSteamGame {
    /// A comma-separated list of sources overlayed over each other.
    ///
    /// Sources on the left have higher priority than sources on the right.
    ///
    /// Can be mixed, some normal directories, other dwarfs archives
    #[arg(long, value_delimiter = ',', required = true)]
    pub(crate) sources: Vec<PathBuf>,
    /// The working directory relative to the root of the overlay of sources.
    #[arg(long, default_value = ".")]
    pub(crate) relative_working_directory: PathBuf,
    #[command(subcommand)]
    pub(crate) run: Run,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Game {
    NonSteam(NonSteamGame),
}

#[derive(Debug, Parser)]
pub(crate) struct CliArgs {
    #[arg(long, default_value_t = false)]
    pub(crate) use_gamescope: bool,
    #[command(subcommand)]
    pub(crate) game: Game,
}
