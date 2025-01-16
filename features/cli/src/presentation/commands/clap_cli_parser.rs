use clap::Parser;

use crate::presentation::commands::Service;

#[derive(Parser, Debug)]
#[command(name = "frosty-pine")]
#[command(version, about, long_about = None)]
pub struct ClapCliParser {
    /// Service to operate on
    #[command(subcommand)]
    pub service: Service,
}
