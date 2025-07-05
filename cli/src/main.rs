mod presentation;

use std::collections::HashMap;

use clap::Parser;
use expense_tracking::domain::repositories::BrandRepository;
use in_memory_storage::application::repositories::BrandRepositoryInMemoryImpl;
use presentation::{FrostyPineCli, clap_args::CliArgs};

#[tokio::main()]
async fn main() {
    let cli_args: CliArgs = CliArgs::parse();
    let brand_repository: Box<dyn BrandRepository> =
        Box::new(BrandRepositoryInMemoryImpl::new(HashMap::new()));

    FrostyPineCli::new(cli_args, brand_repository).run().await
}
