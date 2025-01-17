use std::{pin::Pin, future::Future};
use crate::{DatabaseTransaction, DbBackend, DbErr, ExecResult, QueryResult, Statement, TransactionError};

#[async_trait::async_trait]
pub trait ConnectionTrait: Sync {
    fn get_database_backend(&self) -> DbBackend;

    async fn execute(&self, stmt: Statement) -> Result<ExecResult, DbErr>;

    async fn query_one(&self, stmt: Statement) -> Result<Option<QueryResult>, DbErr>;

    async fn query_all(&self, stmt: Statement) -> Result<Vec<QueryResult>, DbErr>;

    /// Execute the function inside a transaction.
    /// If the function returns an error, the transaction will be rolled back. If it does not return an error, the transaction will be committed.
    async fn transaction<F, T, E>(&self, callback: F) -> Result<T, TransactionError<E>>
    where
        F: for<'c> FnOnce(&'c DatabaseTransaction<'_>) -> Pin<Box<dyn Future<Output = Result<T, E>> + Send + 'c>> + Send + Sync,
        T: Send,
        E: std::error::Error + Send;
}
