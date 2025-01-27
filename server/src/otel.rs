use langdb_ai_gateway::{otel::SpanWriterTransport, GatewayResult};
use serde_json::Value;

pub struct DummyTraceWritterTransport {}

#[async_trait::async_trait]
impl SpanWriterTransport for DummyTraceWritterTransport {
    async fn insert_values(
        &self,
        _table_name: &str,
        _columns: &[&str],
        _body: Vec<Vec<Value>>,
    ) -> GatewayResult<String> {
        Ok("".to_string())
    }
}
