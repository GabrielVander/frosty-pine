use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::Utc;
use clap::{Args, Subcommand};

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct TransactionsArgs {
    #[command(subcommand)]
    command: TransactionCommands,
}

#[derive(Debug, Subcommand)]
enum TransactionCommands {
    Add {
        /// Accepts YYYY-MM-DD
        #[arg(
            short,
            long,
            required_unless_present = "date_time",
            conflicts_with = "date_time",
            value_parser = parse_date
        )]
        date: DateTime<Utc>,

        /// Accepts YYYY-MM-DD hh:mm:ss
        #[arg(short, long, required_unless_present = "date", conflicts_with = "date", value_parser = parse_date_time)]
        date_time: DateTime<Utc>,

        #[arg(
            long,
            required_unless_present = "store_name",
            conflicts_with = "store_name"
        )]
        store_id: Option<String>,

        #[arg(
            long,
            required_unless_present = "store_id",
            conflicts_with = "store_id"
        )]
        store_name: Option<String>,
    },

    Get {
        #[arg(short, long)]
        id: Option<String>,

        #[arg(long)]
        store_id: Option<String>,

        #[arg(long, conflicts_with = "store_id")]
        store_name: Option<String>,
    },

    Update {
        #[arg(short, long)]
        id: String,

        #[arg(long, conflicts_with = "store_name")]
        store_id: Option<String>,

        #[arg(long, conflicts_with = "store_id")]
        store_name: Option<String>,

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

#[derive(Debug, Args)]
struct Item {
    /// Accepts YYYY-MM-DD
    #[arg(
            short,
            long,
            required_unless_present = "date_time",
            conflicts_with = "date_time",
            value_parser = parse_date
        )]
    date: DateTime<Utc>,

    /// Accepts YYYY-MM-DD hh:mm:ss
    #[arg(short, long, required_unless_present = "date", conflicts_with = "date", value_parser = parse_date_time)]
    date_time: DateTime<Utc>,

    #[arg(
        long,
        required_unless_present = "store_name",
        conflicts_with = "store_name"
    )]
    store_id: Option<String>,

    #[arg(
        long,
        required_unless_present = "store_id",
        conflicts_with = "store_id"
    )]
    store_name: Option<String>,

    #[arg(
        long,
        required_unless_present = "product_name",
        conflicts_with = "product_name"
    )]
    product_id: Option<String>,

    #[arg(
        long,
        required_unless_present = "product_id",
        conflicts_with = "product_id"
    )]
    product_name: String,

    #[arg(
        long,
        required_unless_present = "product_brand_name",
        conflicts_with = "product_brand_name"
    )]
    product_brand_id: Option<String>,

    #[arg(
        long,
        required_unless_present = "product_brand_id",
        conflicts_with = "product_brand_id"
    )]
    product_brand_name: Option<String>,

    #[arg(
        long,
        required_unless_present = "product_category_name",
        conflicts_with = "product_category_name"
    )]
    product_category_id: Option<String>,

    #[arg(
        long,
        required_unless_present = "product_category_id",
        conflicts_with = "product_category_id"
    )]
    product_category_name: Option<String>,
    // #[arg(
    //     long,
    //     required_unless_present = "product_category_id",
    //     conflicts_with = "product_category_id"
    // )]
    // unit: Unit,
}

// #[derive(Debug, Args)]
// enum Unit {
//     None,
//     Quantity(f64),
//     Kilograms(f64),
//     Liters(f64),
// }

fn parse_date(arg: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
    parse_arg_to_chrono_date_time(arg, "%Y-%m-%d")
}

fn parse_date_time(arg: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
    parse_arg_to_chrono_date_time(arg, "%Y-%m-%d %H:%M:%S")
}

fn parse_arg_to_chrono_date_time(
    arg: &str,
    pattern: &str,
) -> Result<DateTime<Utc>, chrono::ParseError> {
    NaiveDateTime::parse_from_str(arg, pattern).map(|i| i.and_utc())
}
