use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
};

use tracing::Subscriber;
use tracing_subscriber::registry::{LookupSpan, SpanRef};

use crate::matcher::SpanMatcher;

#[derive(Default)]
pub(crate) struct EntryState {
    created: AtomicUsize,
    entered: AtomicUsize,
    exited: AtomicUsize,
    closed: AtomicUsize,
}

impl EntryState {
    pub fn track_created(&self) {
        self.created.fetch_add(1, Ordering::AcqRel);
    }

    pub fn track_entered(&self) {
        self.entered.fetch_add(1, Ordering::AcqRel);
    }

    pub fn track_exited(&self) {
        self.exited.fetch_add(1, Ordering::AcqRel);
    }

    pub fn track_closed(&self) {
        self.closed.fetch_add(1, Ordering::AcqRel);
    }

    pub fn num_created(&self) -> usize {
        self.created.load(Ordering::Acquire)
    }

    pub fn num_entered(&self) -> usize {
        self.entered.load(Ordering::Acquire)
    }

    pub fn num_exited(&self) -> usize {
        self.exited.load(Ordering::Acquire)
    }

    pub fn num_closed(&self) -> usize {
        self.closed.load(Ordering::Acquire)
    }
}

#[derive(Default)]
pub(crate) struct State {
    entries: Mutex<HashMap<SpanMatcher, Arc<EntryState>>>,
}

impl State {
    pub fn create_entry(&self, matcher: SpanMatcher) -> Arc<EntryState> {
        let mut entries = self
            .entries
            .lock()
            .expect("i literally don't know what a poisoned thread is");
        let entry = entries
            .entry(matcher)
            .or_insert_with(|| Arc::new(EntryState::default()));
        Arc::clone(entry)
    }

    pub fn get_entry<S>(&self, span: SpanRef<'_, S>) -> Option<Arc<EntryState>>
    where
        S: Subscriber + for<'a> LookupSpan<'a>,
    {
        let entries = self
            .entries
            .lock()
            .expect("i literally don't know what a poisoned thread is");
        entries
            .iter()
            .find(|(matcher, _)| matcher.matches(&span))
            .map(|(_, state)| Arc::clone(state))
    }
}
