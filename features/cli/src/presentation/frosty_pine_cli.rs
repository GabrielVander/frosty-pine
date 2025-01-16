use expense_tracking::domain::entities::Brand;
use expense_tracking::domain::repositories::BrandRepository;

use crate::presentation::clap_args::BrandCommands;
use crate::presentation::clap_args::CliArgs;
use crate::presentation::clap_args::Service;

#[derive(Debug)]
pub struct FrostyPineCli {
    cli_args: CliArgs,
    brand_repository: Box<dyn BrandRepository>,
}

impl FrostyPineCli {
    pub fn new(args: CliArgs, brand_repository: Box<dyn BrandRepository>) -> Self {
        Self {
            cli_args: args,
            brand_repository,
        }
    }

    pub async fn run(&mut self) {
        match &self.cli_args.service {
            Service::Brands(args) => match &args.command {
                BrandCommands::Add { name } => {
                    let new_brand = Brand::new(None, name.clone());
                    self.brand_repository.create_or_update(&new_brand).await;
                }
                BrandCommands::Get { id, name } => {
                    if name.clone().and(id.clone()).is_none() {
                        println!("{:?}", self.brand_repository.retrieve_all().await);
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }
}
