
// ===== Imports =====
use std::sync::Arc;
use std::collections::HashMap;
use rand::random;
use tokio::sync::Mutex;
use tokio::time::{Duration, Instant};
use tonic::{Request, Response, Status};
use crate::raft::proto::{
    gnosis_raft_server::GnosisRaft,
    gnosis_raft_client::GnosisRaftClient,
    AppendEntriesReq, AppendEntriesRes,
    RequestVoteReq, RequestVoteRes
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

    pub async fn start(&self) {
        tokio::spawn(run_election_timer(self.clone()));
    }

    pub async fn start_election(&self) {
        let mut state = self.state.lock().await;

        state.election_reset_event = Instant::now();
        state.kind = NodeKind::Candidate;
        state.current_term += 1;
        state.voted_for = Some(state.id);
        let saved_current_term = state.current_term;
        let votes_received = Arc::new(Mutex::new(0));

        let majority_mark = state.peer_ids.len() / 2;

        for &id in &state.peer_ids {
            let peer_url = state.peer_urls[&id].clone();
            let request = RequestVoteReq {
                candidate_id: state.id,
                term: saved_current_term,
                last_log_index: 0, // TODO: Change from hard-coded to actual value
                last_log_term: 0, // TODO: Change from hard-coded to actual value
            };
            let votes_received_mutex = votes_received.clone();
            let mut node = self.clone();

            tokio::spawn(async move {
                match request_vote(&mut node, peer_url, request, saved_current_term).await {
                    Err(_) => {},
                    Ok(result) => {
                        let mut votes_received = votes_received_mutex.lock().await;
                        if result {
                            *votes_received += 1;
                        }

                        if *votes_received > majority_mark {
                            node.start_leader().await;
                        }
                    },
                }
            });
        }
    }

    async fn start_leader(&self) {}

    async fn become_follower(&self) {}
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

/// Returns a `Duration` between 150-300ms
fn get_election_timeout_duration() -> Duration {
    Duration::from_millis(150 + ((150.0 * random::<f64>()) as u64))
}

/// Runs an election timer which starts a new election from the given node.
/// This method should be run in a separate thread with a clone of the node.
pub async fn run_election_timer(node: Node) {
    let term_started = {
        let state = node.state.lock().await;
        state.current_term
    };

    let election_timeout = get_election_timeout_duration();

    // event loop
    let mut ticker = tokio::time::interval(Duration::from_millis(10));
    loop {
        ticker.tick().await;
        let state = node.state.lock().await;

        if let NodeKind::Leader { .. } = state.kind {
            drop(state);
            return;
        }

        if state.current_term != term_started {
            drop(state);
            return;
        }

        if Instant::now().duration_since(state.election_reset_event) >= election_timeout {
            drop(state);
            // node.start_election().await;
            return;
        }

        drop(state);
    }
}

pub async fn request_vote(
    node: &mut Node,
    peer_url: String,
    request: RequestVoteReq,
    saved_current_term: u32,
) -> anyhow::Result<bool> {
    let mut client = GnosisRaftClient::connect(peer_url).await?;
    let result = client.request_vote(request).await?.into_inner();

    let state = node.state.lock().await;

    match state.kind {
        NodeKind::Leader { .. } | NodeKind::Follower => {
            drop(state);
            return Ok(false);
        },
        _ => {},
    }

    if result.term > saved_current_term {
        drop(state);
        return Ok(false);
    }

    if result.term == saved_current_term && result.vote_granted {
        drop(state);
        return Ok(true);
    }

    drop(state);
    Ok(false)
}