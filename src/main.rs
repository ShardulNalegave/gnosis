
pub mod raft;

use std::time::Duration;

use tonic::transport::Server;
use raft::MyGreeter;
use raft::gnosis_raft::gnosis_raft_server::GnosisRaftServer;
use raft::gnosis_raft::gnosis_raft_client::GnosisRaftClient;
use raft::gnosis_raft::HelloRequest;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "[::1]:50051".parse().unwrap();
    let greeter = MyGreeter::default();

    println!("GreeterServer listening on {addr}");

    let task = Server::builder()
        .add_service(GnosisRaftServer::new(greeter))
        .serve(addr);

    tokio::time::sleep(Duration::from_secs(5)).await;

    tokio::spawn(task);

    let mut client = GnosisRaftClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let response = client.say_hello(request).await?;

    tokio::time::sleep(Duration::from_secs(5)).await;

    println!("RESPONSE={response:?}");
    Ok(())
}