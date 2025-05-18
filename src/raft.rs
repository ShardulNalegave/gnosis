
use tonic::{Request, Response, Status};

use gnosis_raft::gnosis_raft_server::GnosisRaft;
use gnosis_raft::{HelloReply, HelloRequest};

pub mod gnosis_raft {
    tonic::include_proto!("gnosis_raft");
}

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl GnosisRaft for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = gnosis_raft::HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }
}