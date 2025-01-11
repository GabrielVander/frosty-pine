use clap::{Args, Subcommand};

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct ProductsArgs {
    #[command(subcommand)]
    command: ProductCommands,
}

#[derive(Debug, Subcommand)]
enum ProductCommands {
    Add {
        #[arg(short, long)]
        name: String,

        #[arg(
            long,
            required_unless_present = "brand_name",
            conflicts_with = "brand_name"
        )]
        brand_id: Option<String>,

        #[arg(
            long,
            required_unless_present = "brand_id",
            conflicts_with = "brand_id"
        )]
        brand_name: Option<String>,

        #[arg(
            long,
            required_unless_present = "category_name",
            conflicts_with = "category_name"
        )]
        category_id: Option<String>,

        #[arg(
            long,
            required_unless_present = "category_id",
            conflicts_with = "category_id"
        )]
        category_name: Option<String>,
    },

    Get {
        #[arg(short, long, required_unless_present = "name", conflicts_with = "name")]
        id: Option<String>,

        #[arg(short, long, required_unless_present = "id", conflicts_with = "id")]
        name: Option<String>,

        #[arg(long)]
        brand_id: Option<String>,

        #[arg(long, conflicts_with = "brand_id")]
        brand_name: Option<String>,

        #[arg(long)]
        category_id: Option<String>,

        #[arg(long, conflicts_with = "category_id")]
        category_name: Option<String>,
    },

    Update {
        #[arg(short, long, required_unless_present = "name", conflicts_with = "name")]
        id: Option<String>,

        #[arg(short, long, required_unless_present = "id", conflicts_with = "id")]
        name: Option<String>,

        #[arg(long, conflicts_with = "brand_name")]
        brand_id: Option<String>,

        #[arg(long, conflicts_with = "brand_id")]
        brand_name: Option<String>,

        #[arg(long, conflicts_with = "category_name")]
        category_id: Option<String>,

        #[arg(long, conflicts_with = "category_id")]
        category_name: Option<String>,
    },

    Delete {
        #[arg(short, long, required_unless_present = "name", conflicts_with = "name")]
        id: Option<String>,

        #[arg(short, long, required_unless_present = "id", conflicts_with = "id")]
        name: Option<String>,
    },
}
