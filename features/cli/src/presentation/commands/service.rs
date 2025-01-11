use crate::presentation::commands::BrandsArgs;
use crate::presentation::commands::CategoriesArgs;
use crate::presentation::commands::ProductsArgs;
use crate::presentation::commands::StoresArgs;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Service {
    /// Operates on Brands
    Brands(BrandsArgs),
    /// Operates on Categories
    Categories(CategoriesArgs),
    /// Operates on Products
    Products(ProductsArgs),
    /// Operates on Stores
    Stores(StoresArgs),
    /// Operates on Transactions
    Transactions,
}
