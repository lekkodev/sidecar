use std::sync::{RwLock, Arc, atomic::AtomicUsize};
use tokio::sync::watch::{Sender, Receiver, self};
use crate::{types::ConnectionCredentials, gen::mod_cli::lekko::backend::v1beta1::RepositoryKey};


// This state transitions linearly, and may skip steps.
// Transitions are governed by StateStore.
// Both Static and Default mode states follow the same state
// machines. We could split it out in the future.
pub enum StateMachine {
    Uninitialized,
    Boostraped(RepositoryKey),
    Active(ConnectionCredentials),
    Shutdown,
}

// The StateStore is meant to encompass any concurrent state outside
// of the configuration. It will govern transitioning states and notifying
// customers of those changes. You can clone StateStore, since it manages
// shared state internally.
#[derive(Clone)]
pub struct StateStore {
    state: Arc<RwLock<StateMachine>>,
    sender: Arc<RwLock<Sender<StateMachine>>>,
    receiver: Receiver<StateMachine>,
    register_counter: Arc<AtomicUsize>,
}

impl StateStore {

    pub fn new(boostrap: Option<RepositoryKey>) -> Self {
	let state = match boostrap {
		None => StateMachine::Uninitialized,
		Some(rk) => StateMachine::Boostraped(rk),
	};
	let (tx, rx) = watch::channel(state);
	return StateStore{
	    state: Arc::new(RwLock::from(state)),
	    sender: Arc::new(RwLock::from(tx)),
	    receiver: rx,
	    register_counter: Arc::new(AtomicUsize::new(0))
	}
    }
    
    pub async fn register(&self) {
	self.register_counter.fetch_add(1,std::sync::atomic::Ordering::Relaxed);
    }
    pub async fn deregister(&self) {
	self.register_counter.fetch_sub(1,std::sync::atomic::Ordering::Relaxed);
    }

    pub async fn shutdown(&self) {
	
    }
    pub fn notify(&self) -> Receiver<StateMachine> {
	return self.receiver.clone()
    }
}
