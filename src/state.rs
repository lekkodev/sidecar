use crate::{
    gen::mod_cli::lekko::backend::v1beta1::RepositoryKey,
    types::{ConnectionCredentials, Mode},
};
use core::ops::Deref;
use log::log;
use std::{
    fmt::Debug,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc,
    },
};
use tokio::sync::watch::{self, Receiver, Sender};
use tonic::metadata::{Ascii, MetadataValue};
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
            Self::Active(cc) => f.write_fmt(format_args!(
                "Active{{repo_key: {:?}, session_key: {}}}",
                cc.repo_key, cc.session_key
            )),
        }
    }
}

// The StateStore is meant to encompass any concurrent state outside
// of the configuration. It will govern transitioning states and notifying
// customers of those changes. You can clone StateStore, since it manages
// shared state internally.
#[derive(Clone)]
pub struct StateStore {
    sender: Arc<Sender<StateMachine>>,
    receiver: Receiver<StateMachine>,
    register_counter: Arc<AtomicUsize>,
    received_shutdown: Arc<AtomicBool>,
    mode: Mode,
}

impl StateStore {
    pub fn new(boostrap: Option<(RepositoryKey, String)>, mode: Mode) -> Self {
        let state = match boostrap {
            None => StateMachine::Uninitialized,
            Some(rk) => StateMachine::Bootstrapped(rk),
        };
        log!(
            log::max_level().to_level().unwrap_or(log::Level::Warn),
            "initializing with state: {state:?}"
        );

        let (tx, rx) = watch::channel(state);
        StateStore {
            sender: Arc::new(tx),
            receiver: rx,
            register_counter: Arc::new(AtomicUsize::new(0)),
            received_shutdown: Arc::new(AtomicBool::new(false)),
            mode,
        }
    }

    pub async fn register(
        &self,
        repo_key: RepositoryKey,
        api_key: MetadataValue<Ascii>,
        session_key: String,
    ) {
        self.register_counter.fetch_add(1, ORDERING);
        if matches!(self.mode, Mode::Default) {
            // Do a state transition, only if we aren't already active.
            if self.sender.send_if_modified(|state| match state {
                StateMachine::Uninitialized | StateMachine::Bootstrapped(_) => {
                    *state = StateMachine::Active(ConnectionCredentials {
                        repo_key,
                        api_key,
                        session_key,
                    });
                    true
                }
                _ => false,
            }) {
                log!(
                    log::max_level().to_level().unwrap_or(log::Level::Warn),
                    "transitioned to Active state",
                );
            }
        }
    }

    pub fn deregister(&self) {
        let res = self.register_counter.fetch_sub(1, ORDERING);
        if self.received_shutdown.load(ORDERING) {
            // If ths previous value was one, thus we decremented to zero, we should shutdown.
            if res == 1
                && self.sender.send_if_modified(|state| match state {
                    StateMachine::Shutdown => false,
                    _ => {
                        *state = StateMachine::Shutdown;
                        true
                    }
                })
            {
                log!(
                    log::max_level().to_level().unwrap_or(log::Level::Warn),
                    "transitioned to Shutdown state",
                );
            }
        }
    }

    // This should be called only after receiving a SIGTERM.
    pub fn shutdown(&self) {
        self.received_shutdown.store(true, ORDERING);
        // Because we shutdown first, and then check ordering, and since
        // the opposite occurs in deregister, no ordering of these 4 checks
        // will result in us missing the shutdown.
        if self.register_counter.load(ORDERING) == 0
            && self.sender.send_if_modified(|state| match state {
                StateMachine::Shutdown => false,
                _ => {
                    *state = StateMachine::Shutdown;
                    true
                }
            })
        {
            log!(
                log::max_level().to_level().unwrap_or(log::Level::Warn),
                "transitioned to Shutdown state",
            );
        }
    }

    // By using the receiver, you can wait on a specific state transition to occur:
    // state_store.receiver().wait_for(|state| matches!(state, StateMachine::Shutdown)).await
    // However this presents some danger, since the value that is availabe through the receiver
    // holds a read lock. We need to care of the borrows here, even if this program doesn't use !Send
    // futures. In the future we could abstract away this notification system, but for now just use
    // the current_state function which creates a copy and holds a short read lock.
    pub fn receiver(&self) -> Receiver<StateMachine> {
        self.receiver.clone()
    }

    pub fn current_state(&self) -> StateMachine {
        return self.receiver.borrow().deref().clone();
    }
}
