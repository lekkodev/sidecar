use std::{sync::{RwLock, Arc, atomic::{AtomicUsize, Ordering}}, fmt::Debug};
use tokio::sync::watch::{Sender, Receiver, self};
use crate::{types::ConnectionCredentials, gen::mod_cli::lekko::backend::v1beta1::RepositoryKey};
use core::ops::Deref;
const ORDERING: Ordering = std::sync::atomic::Ordering::Relaxed;

// This state transitions linearly, and may skip steps.
// Transitions are governed by StateStore.
// Both Static and Default mode states follow the same state
// machines. We could split it out in the future.
#[derive(Clone)]
pub enum StateMachine {
    Uninitialized,
    // Contains the repo key and bootstrapped sha.
    Bootstrapped((RepositoryKey, String)),
    Active(ConnectionCredentials),
    Shutdown,
}

impl Debug for StateMachine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
	match self {
	    Self::Uninitialized => f.write_str("Uninitialized"),
	    Self::Shutdown => f.write_str("Shutdown"),
	    Self::Bootstrapped(rk) => f.write_fmt(format_args!("Bootstrap{{repo_key: {:?}}}", rk)),
	    Self::Active(cc) => f.write_fmt(format_args!("Active{{repo_key: {:?}, session_key: {}}}", cc.repo_key, cc.session_key)),
	}
    }
}

// The StateStore is meant to encompass any concurrent state outside
// of the configuration. It will govern transitioning states and notifying
// customers of those changes. You can clone StateStore, since it manages
// shared state internally.
#[derive(Clone)]
pub struct StateStore {
    sender: Arc<RwLock<Sender<StateMachine>>>,
    receiver: Receiver<StateMachine>,
    register_counter: Arc<AtomicUsize>,
}

impl StateStore {
    pub fn new(boostrap: Option<(RepositoryKey, String)>) -> Self {
	let state = match boostrap {
		None => StateMachine::Uninitialized,
		Some(rk) => StateMachine::Bootstrapped(rk),
	};
	let (tx, rx) = watch::channel(state);
	return StateStore{
	    sender: Arc::new(RwLock::from(tx)),
	    receiver: rx,
	    register_counter: Arc::new(AtomicUsize::new(0))
	}
    }
    
    pub async fn register(&self, _session_key: String) {
	self.register_counter.fetch_add(1,ORDERING);
	// TODO(konrad) state change
    }

    pub async fn deregister(&self) {
	self.register_counter.fetch_sub(1,ORDERING);
	
	if self.register_counter.load(ORDERING) == 0 {
	    // TODO(konrad): upon sigterm only.
	    self.sender.write().unwrap().send(StateMachine::Shutdown).unwrap()
	}
    }

    pub async fn shutdown(&self) {
	
    }

    // By using the receiver, you can wait on a specific state transition to occur:
    // state_store.receiver().wait_for(|state| matches!(state, StateMachine::Shutdown)).await
    // However this presents some danger, since the value that is availabe through the receiver
    // holds a read lock. We need to care of the borrows here, even if this program doesn't use !Send
    // futures. In the future we could abstract away this notification system, but for now just use
    // the current_state function which creates a copy and holds a short read lock.
    pub fn receiver(&self) -> Receiver<StateMachine> {
	return self.receiver.clone()
    }

    pub fn current_state(&self) -> StateMachine {
	return self.receiver.borrow().deref().clone()
    }
}
