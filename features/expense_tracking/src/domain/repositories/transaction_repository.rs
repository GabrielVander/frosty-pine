use async_trait::async_trait;

use crate::domain::entities::Transaction;

#[async_trait]
pub trait TransactionRepository {
    async fn create_or_update(
        &self,
        category: &Transaction,
    ) -> Result<Option<Transaction>, TransactionRepositoryError>;

    async fn retrieve_all(&self) -> Result<Vec<Transaction>, TransactionRepositoryError>;
}

#[derive(Debug, PartialEq)]
pub enum TransactionRepositoryError {}
