
// ===== Imports =====
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use tokio::time::{Duration, Instant};
use rand::random;
use tonic::{Request, Response, Status};
use raft::{
    RequestVoteReq, RequestVoteRes,
    AppendEntriesReq, AppendEntriesRes,
    gnosis_raft_server::GnosisRaft,
    gnosis_raft_client::GnosisRaftClient,
};
// ===================

pub mod raft {
    tonic::include_proto!("gnosis_raft");
}

/// Returns a `Duration` between 150-300ms
fn get_election_timeout_duration() -> Duration {
    Duration::from_millis(150 + ((150.0 * random::<f64>()) as u64))
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodeKind {
    Follower,
    Candidate,
    Leader {
        next_index: HashMap<u32, u32>,
        match_index: HashMap<u32, u32>,
    },
}

#[derive(Clone, Debug)]
pub struct TNodeState {
    kind: NodeKind,
    current_term: u32,
    voted_for: Option<u32>,

    election_reset_event: Instant,
}

type NodeState = Arc<Mutex<TNodeState>>;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Node {
    id: u32,
    peer_ids: Vec<u32>,
    peer_urls: HashMap<u32, String>,
    state: NodeState,
}

impl Node {
    pub fn new() -> Self {
        unimplemented!()
    }

    pub async fn start(&self) {
        self.run_election_timer().await;
    }

    async fn run_election_timer(&self) {
        let term_started = match self.state.lock() {
            Ok(state) => state.current_term,
            Err(_) => unreachable!(),
        };

        let election_timeout = get_election_timeout_duration();

        let mut event_loop_timer = tokio::time::interval(
            Duration::from_millis(10),
        );

        loop {
            event_loop_timer.tick().await;

            let state = match self.state.lock() {
                Ok(state) => state,
                Err(_) => unreachable!(),
            };

            // already a leader, no need for a election timer
            if let NodeKind::Leader { .. } = state.kind {
                drop(state);
                return;
            }

            // the term has changed, bail out
            if term_started != state.current_term {
                drop(state);
                return;
            }

            // we haven't heard from the leader for a duration more than the election timer, start a new election
            if Instant::now().duration_since(state.election_reset_event) >= election_timeout {
                drop(state);
                self.start_election().await;
                return;
            }

            drop(state);
        }
    }

    async fn start_election(&self) {
        let mut state = match self.state.lock() {
            Ok(state) => state,
            Err(_) => unreachable!(),
        };

        state.election_reset_event = Instant::now();
        state.kind = NodeKind::Candidate;
        state.current_term += 1;
        state.voted_for = Some(self.id);
        let votes_received = 1;

        for id in &self.peer_ids {
            // TODO: Send `RequestVote` RPCs to all member nodes

            let peer_url = self.peer_urls[id].clone();
            let request = RequestVoteReq {
                candidate_id: self.id,
                term: state.current_term,
                last_log_index: 0, // TODO: Change from hard-coded to actual value
                last_log_term: 0, // TODO: Change from hard-coded to actual value
            };
        }
    }
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