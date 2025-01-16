mod presentation;

use std::collections::HashMap;

use clap::Parser;
use expense_tracking::{
    data::repositories::BrandRepositoryInMemoryImpl,
    domain::{entities::Brand, repositories::BrandRepository},
};
use presentation::{BrandCommands, BrandsArgs, ClapCliParser, Service};

#[tokio::main()]
async fn main() {
    let clap_args: ClapCliParser = ClapCliParser::parse();
    let brand_repository: Box<BrandRepositoryInMemoryImpl> =
        Box::new(BrandRepositoryInMemoryImpl::new(HashMap::new()));

    FrostyPineCli::new(clap_args, brand_repository).run().await
}

#[derive(Debug)]
struct FrostyPineCli {
    clap_args: ClapCliParser,
    brand_repository: Box<dyn BrandRepository>,
}

impl FrostyPineCli {
    fn new(clap_parser: ClapCliParser, brand_repository: Box<dyn BrandRepository>) -> Self {
        Self {
            clap_args: clap_parser,
            brand_repository,
        }
    }

    pub async fn run(&mut self) {
        match &self.clap_args.service {
            Service::Brands(args) => match &args.command {
                BrandCommands::Add { name } => {
                    let new_brand = Brand::new(None, name.clone());
                    self.brand_repository.create_or_update(&new_brand).await;
                    ()
                }
                BrandCommands::Get { id, name } => {
                    if (name.clone().and(id.clone()).is_none()) {
                        println!("{:?}", self.brand_repository.retrieve_all().await);
                    }
                    ()
                }
                _ => {}
            },
            _ => {}
        }
    }
}
