
// ===== Imports =====
use rand::random;
use tokio::time::{Duration, Instant};
use crate::node::{Node, NodeKind};
// ===================

/// Returns a `Duration` between 150-300ms
fn get_election_timeout_duration() -> Duration {
    Duration::from_millis(150 + ((150.0 * random::<f64>()) as u64))
}

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