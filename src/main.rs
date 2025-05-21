
pub mod raft;
pub mod node;
pub mod election;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Ok(())
}