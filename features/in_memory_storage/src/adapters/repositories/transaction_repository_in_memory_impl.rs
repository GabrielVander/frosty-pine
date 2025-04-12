use std::collections::HashMap;

use async_trait::async_trait;
use uuid_b64::UuidB64;

use expense_tracking::domain::{
    entities::Transaction,
    repositories::{TransactionRepository, TransactionRepositoryError},
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TransactionRepositoryInMemoryImpl {
    hash_map: HashMap<UuidB64, Transaction>,
}

impl TransactionRepositoryInMemoryImpl {
    pub fn new(hash_map: HashMap<UuidB64, Transaction>) -> Self {
        Self { hash_map }
    }
}

#[async_trait]
impl TransactionRepository for TransactionRepositoryInMemoryImpl {
    async fn create_or_update(
        &mut self,
        transaction: &Transaction,
    ) -> Result<Option<Transaction>, TransactionRepositoryError> {
        Ok(self.hash_map.insert(transaction.id, transaction.clone()))
    }

    async fn retrieve_all(&self) -> Result<Vec<Transaction>, TransactionRepositoryError> {
        Ok(self.hash_map.values().cloned().collect())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use uuid_b64::UuidB64;

    use super::TransactionRepositoryInMemoryImpl;
    use chrono::DateTime;
    use expense_tracking::domain::{
        entities::{Item, Store, Transaction},
        repositories::{TransactionRepository, TransactionRepositoryError},
    };

    fn given_empty_repository() -> TransactionRepositoryInMemoryImpl {
        given_repository_with(Vec::new())
    }

    fn given_repository_with(transactions: Vec<Transaction>) -> TransactionRepositoryInMemoryImpl {
        TransactionRepositoryInMemoryImpl::new(
            transactions
                .into_iter()
                .map(|b| (b.id, b))
                .collect::<HashMap<UuidB64, Transaction>>(),
        )
    }

    fn given_new_item() -> Item {
        Item::default()
    }
    fn given_new_store() -> Store {
        Store::default()
    }

    fn given_new_transaction(items: Vec<Item>) -> Transaction {
        Transaction::new(None, items, given_new_store(), DateTime::default())
    }

    macro_rules! retrieve_all {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[tokio::test]
            async fn $name() {
                let transactions: &Vec<Transaction> = $value;

                let repository: TransactionRepositoryInMemoryImpl = given_repository_with(transactions.clone());

                let result: Result<Vec<Transaction>, TransactionRepositoryError> =
                    repository.retrieve_all().await;

                assert!(result.is_ok(), "Expected Ok, got {:?}", result);

                let mut expected_transactions: Vec<Transaction> = transactions.clone();
                expected_transactions.sort_by(|a, b| a.id.cmp(&b.id));

                let mut sorted_result: Vec<Transaction> = result.unwrap();
                sorted_result.sort_by(|a, b| a.id.cmp(&b.id));

                assert_eq!(
                    sorted_result, expected_transactions,
                    "Expected {:?}, but got {:?}",
                    expected_transactions, sorted_result
                )
            }
        )*
        }
    }

    retrieve_all! {
        no_transactions: &vec![],
        one_transaction_no_items: &vec![given_new_transaction(vec![])],
        one_transaction_one_item: &vec![given_new_transaction(vec![given_new_item()])],
        one_transaction_multiple_items: &vec![given_new_transaction(vec![given_new_item(), given_new_item(), given_new_item()])],
        multiple_transactions_no_items: &vec![given_new_transaction(vec![]), given_new_transaction(vec![]), given_new_transaction(vec![])],
        multiple_transactions_one_item_each: &vec![given_new_transaction(vec![given_new_item()]), given_new_transaction(vec![given_new_item()]), given_new_transaction(vec![given_new_item()])],
        multiple_transactions_multiple_items_each: &vec![given_new_transaction(vec![given_new_item(), given_new_item(), given_new_item()]), given_new_transaction(vec![given_new_item(), given_new_item(), given_new_item()]), given_new_transaction(vec![given_new_item(), given_new_item(), given_new_item()])],
    }

    #[tokio::test]
    async fn create_or_update_should_add_new_transaction_given_empty_repository() {
        let transaction: Transaction = Transaction::new(
            None,
            vec![given_new_item(), given_new_item(), given_new_item()],
            given_new_store(),
            DateTime::default(),
        );
        let mut repository: TransactionRepositoryInMemoryImpl = given_empty_repository();

        let result: Result<Option<Transaction>, TransactionRepositoryError> =
            repository.create_or_update(&transaction).await;
        let expected: Result<Option<Transaction>, TransactionRepositoryError> = Ok(None);

        assert_eq!(
            result, expected,
            "Expected {:?}, but got {:?}",
            expected, result
        );
    }

    #[tokio::test]
    async fn create_or_update_should_add_new_transaction_given_full_repository() {
        let transaction: Transaction = Transaction::new(
            None,
            vec![given_new_item(), given_new_item(), given_new_item()],
            given_new_store(),
            DateTime::default(),
        );
        let mut repository: TransactionRepositoryInMemoryImpl = given_repository_with(vec![
            given_new_transaction(vec![given_new_item(), given_new_item(), given_new_item()]),
            given_new_transaction(vec![given_new_item(), given_new_item(), given_new_item()]),
            given_new_transaction(vec![given_new_item(), given_new_item(), given_new_item()]),
        ]);

        let result: Result<Option<Transaction>, TransactionRepositoryError> =
            repository.create_or_update(&transaction).await;
        let expected: Result<Option<Transaction>, TransactionRepositoryError> = Ok(None);

        assert_eq!(
            result, expected,
            "Expected {:?}, but got {:?}",
            expected, result
        );
    }

    #[tokio::test]
    async fn create_or_update_should_update_transaction_given_full_repository() {
        let old_transaction: Transaction = Transaction::new(
            None,
            vec![given_new_item(), given_new_item(), given_new_item()],
            given_new_store(),
            DateTime::default(),
        );
        let updated_transaction: Transaction = Transaction::new(
            Some(old_transaction.id),
            vec![],
            old_transaction.store.clone(),
            old_transaction.datetime,
        );

        let mut repository: TransactionRepositoryInMemoryImpl = given_repository_with(vec![
            given_new_transaction(vec![given_new_item(), given_new_item(), given_new_item()]),
            old_transaction.clone(),
            given_new_transaction(vec![given_new_item(), given_new_item(), given_new_item()]),
        ]);

        let result: Result<Option<Transaction>, TransactionRepositoryError> =
            repository.create_or_update(&updated_transaction).await;
        let expected: Result<Option<Transaction>, TransactionRepositoryError> =
            Ok(Some(old_transaction));

        assert_eq!(
            result, expected,
            "Expected {:?}, but got {:?}",
            expected, result
        );
        let mut transactions = repository.retrieve_all().await.unwrap();
        transactions.retain(|e| e.id == updated_transaction.id);
        assert_eq!(
            transactions[0], updated_transaction,
            "Updated transaction not found"
        );
    }
}
