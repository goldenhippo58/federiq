use federated_query_engine::api::run_api_server;
use tokio::runtime::Runtime;

fn main() {
    let rt = Runtime::new().expect("Failed to create Tokio runtime");
    rt.block_on(run_api_server());
}
