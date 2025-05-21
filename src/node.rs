
// ===== Imports =====
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::Mutex;
use tokio::time::Instant;
use tonic::{Request, Response, Status};
use crate::raft::proto::{
    RequestVoteReq, RequestVoteRes,
    AppendEntriesReq, AppendEntriesRes,
    gnosis_raft_server::GnosisRaft,
};
// ===================

/// # Node Kind
/// The different possible states for any node along with the state they need.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodeKind {
    Follower,
    Candidate,
    Leader {
        next_index: HashMap<u32, u32>,
        match_index: HashMap<u32, u32>,
    },
}

/// # Node State
/// Struct for storing all information/state about nodes.
/// This struct is not meant to be used directly, this is only a container for all the data.
/// All methods, etc are implemented on the `Node` handler struct which holds a `Arc<Mutex<NodeState>>` and is this cheaply clonable.
#[derive(Clone, Debug)]
pub struct NodeState {
    pub id: u32,
    pub peer_ids: Vec<u32>,
    pub peer_urls: HashMap<u32, String>,

    pub kind: NodeKind,
    pub current_term: u32,
    pub voted_for: Option<u32>,
    pub election_reset_event: Instant,
}

/// # Node
/// Struct implementing all Node related functionality along with handlers for the gRPC services.
/// It is a cheaply clonable struct containing only a `Arc<Mutex<NodeState>>`
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Node {
    pub state: Arc<Mutex<NodeState>>,
}

impl Node {
    pub fn new() -> Self {
        unimplemented!()
    }

    // pub async fn start(&self) {
    //     self.run_election_timer().await;
    // }

    // async fn run_election_timer(&self) {
    //     let term_started = self.state.lock().await.current_term;

    //     let election_timeout = get_election_timeout_duration();

    //     let mut event_loop_timer = tokio::time::interval(
    //         Duration::from_millis(10),
    //     );

    //     loop {
    //         event_loop_timer.tick().await;

    //         let state = self.state.lock().await;

    //         // already a leader, no need for a election timer
    //         if let NodeKind::Leader { .. } = state.kind {
    //             drop(state);
    //             return;
    //         }

    //         // the term has changed, bail out
    //         if term_started != state.current_term {
    //             drop(state);
    //             return;
    //         }

    //         // we haven't heard from the leader for a duration more than the election timer, start a new election
    //         if Instant::now().duration_since(state.election_reset_event) >= election_timeout {
    //             drop(state);
    //             // self.start_election().await;
    //             return;
    //         }

    //         drop(state);
    //     }
    // }

    // pub async fn start_election(&self) {
    //     let mut state = self.state.lock().await;

    //     state.election_reset_event = Instant::now();
    //     state.kind = NodeKind::Candidate;
    //     state.current_term += 1;
    //     state.voted_for = Some(self.id);
    //     let saved_current_term = state.current_term;
    //     let mut votes_received = Arc::new(Mutex::new(0));

    //     for id in &self.peer_ids {
    //         // TODO: Send `RequestVote` RPCs to all member nodes

    //         let peer_url = self.peer_urls[id].clone();
    //         let request = RequestVoteReq {
    //             candidate_id: self.id,
    //             term: saved_current_term,
    //             last_log_index: 0, // TODO: Change from hard-coded to actual value
    //             last_log_term: 0, // TODO: Change from hard-coded to actual value
    //         };
    //         let state_mutex = self.state.clone();
    //         let votes_received_mutex = votes_received.clone();
    //         let num_peers = self.peer_ids.len();

    //         tokio::spawn(async move {
    //             let mut client = GnosisRaftClient::connect(peer_url).await.unwrap();
    //             let result = client.request_vote(request).await.unwrap().into_inner();
    //             let state = state_mutex.lock().await;

    //             match state.kind {
    //                 NodeKind::Candidate | NodeKind::Follower => {
    //                     drop(state);
    //                     return;
    //                 },
    //                 _ => {},
    //             }

    //             if result.term > saved_current_term {
    //                 drop(state);
    //                 return;
    //             }

    //             if result.term == saved_current_term && result.vote_granted {
    //                 let mut votes_received = votes_received_mutex.lock().await;
    //                 *votes_received += 1;

    //                 if 2 * *votes_received > num_peers + 1 {
    //                     drop(votes_received);
    //                     drop(state);
    //                     return;
    //                 }
    //             }

    //             drop(state);
    //         });
    //     }
    // }

    // async fn become_follower(&self) {}
    // async fn start_leader(&self) {}
}

#[tonic::async_trait]
impl GnosisRaft for Node {
    async fn request_vote(
        &self,
        _request: Request<RequestVoteReq>,
    ) -> Result<Response<RequestVoteRes>, Status> {
        unimplemented!()
    }

    async fn append_entries(
        &self,
        _request: Request<AppendEntriesReq>,
    ) -> Result<Response<AppendEntriesRes>, Status> {
        unimplemented!()
    }
}