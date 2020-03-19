use async_trait::async_trait;

use cassandra_proto::{error, types::CBytesShort};

pub type PreparedQuery = CBytesShort;

#[async_trait]
pub trait PrepareExecutor {
  /// It prepares a query for execution, along with query itself
  /// the method takes `with_tracing` and `with_warnings` flags
  /// to get tracing information and warnings.
  async fn prepare_tw<Q: ToString + Send>(
    &self,
    query: Q,
    with_tracing: bool,
    with_warnings: bool,
  ) -> error::Result<PreparedQuery>;

  /// It prepares query without additional tracing information and warnings.
  async fn prepare<Q: ToString + Send>(&self, query: Q) -> error::Result<PreparedQuery> {
    self.prepare_tw(query, false, false).await
  }
}
