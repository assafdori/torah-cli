use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "torah", about = "A Torah TUI for terminal reading")]
#[command(version, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Skip the startup banner animation
    #[arg(long, global = true)]
    pub no_banner: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Read a Torah verse, range, or chapter (e.g. "Genesis 1:1", "Exodus 3", "Deut 6:4-9")
    Read {
        /// Torah reference (e.g. "Genesis 1:1", "Exodus 3")
        #[arg(required = true, num_args = 1..)]
        reference: Vec<String>,

        /// Translation (default: TORAH)
        #[arg(short, long, default_value = "TORAH")]
        translation: String,
    },

    /// Search the Torah for a phrase or keyword
    Search {
        /// Search query
        #[arg(required = true, num_args = 1..)]
        query: Vec<String>,

        /// Translation to search in
        #[arg(short, long, default_value = "TORAH")]
        translation: String,
    },

    /// Display a random Torah verse
    Random {
        /// Translation
        #[arg(short, long, default_value = "TORAH")]
        translation: String,
    },

    /// Show today's verse
    Today {
        /// Translation
        #[arg(short, long, default_value = "TORAH")]
        translation: String,
    },

    /// Replay the startup animation
    Intro,

    /// Update torah-cli to the latest version
    Update {
        /// Check for updates without installing
        #[arg(long)]
        check: bool,
    },
}
