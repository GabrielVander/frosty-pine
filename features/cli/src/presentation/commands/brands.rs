use clap::{Args, Subcommand};

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct BrandsArgs {
    #[command(subcommand)]
    command: BrandCommands,
}

#[derive(Debug, Subcommand)]
enum BrandCommands {
    Add {
        #[arg(short, long)]
        name: String,
    },

    Get {
        #[arg(short, long, required_unless_present = "name", conflicts_with = "name")]
        id: Option<String>,

        #[arg(short, long, required_unless_present = "id", conflicts_with = "id")]
        name: Option<String>,
    },

    Update {
        #[arg(short, long)]
        id: String,

        #[arg(short, long)]
        name: String,
    },

    Delete {
        #[arg(short, long, required_unless_present = "name", conflicts_with = "name")]
        id: Option<String>,

        #[arg(short, long, required_unless_present = "id", conflicts_with = "id")]
        name: Option<String>,
    },
}
