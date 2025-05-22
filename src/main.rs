
pub mod raft;
pub mod node;

// ===== Imports =====
#[macro_use] extern crate tracing;
// ===================

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    Ok(())
}