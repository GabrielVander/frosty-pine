use clap::Parser;

use crate::presentation::commands::Service;

#[derive(Parser)]
#[command(name = "frosty-pine")]
#[command(version, about, long_about = None)]
pub struct FrostyPine {
    /// Service to operate on
    #[command(subcommand)]
    service: Option<Service>,
}
