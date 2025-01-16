mod presentation;

use std::collections::HashMap;

use clap::Parser;
use expense_tracking::{
    data::repositories::BrandRepositoryInMemoryImpl, domain::repositories::BrandRepository,
};
use presentation::{FrostyPineCli, clap_args::CliArgs};

#[tokio::main()]
async fn main() {
    let cli_args: CliArgs = CliArgs::parse();
    let brand_repository: Box<dyn BrandRepository> =
        Box::new(BrandRepositoryInMemoryImpl::new(HashMap::new()));

    FrostyPineCli::new(cli_args, brand_repository).run().await
}
