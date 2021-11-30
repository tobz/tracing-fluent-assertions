use std::{any::TypeId, marker::PhantomData, sync::Arc};

use tracing::{span::Attributes, Id, Subscriber};
use tracing_subscriber::{layer::Context, registry::LookupSpan, Layer};

use crate::{state::State, AssertionRegistry};

/// [`FluentAssertionsLayer`] is a [`tracing_subscriber::Layer`] that tracks assertions as spans
/// transition through the various states of their lifecycle.
pub struct FluentAssertionsLayer<S> {
    state: Arc<State>,
    _subscriber: PhantomData<fn(S)>,
}

impl<S> FluentAssertionsLayer<S>
where
    S: Subscriber,
{
    /// Create a new `FluentAssertionsLayer`.
    pub fn new(controller: &AssertionRegistry) -> Self {
        Self {
            state: Arc::clone(controller.state()),
            _subscriber: PhantomData,
        }
    }
}

impl<S> Layer<S> for FluentAssertionsLayer<S>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    fn new_span(&self, _attributes: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
        let span = ctx.span(id).expect("span must already exist!");
        if let Some(entry) = self.state.get_entry(span) {
            entry.track_created();
        }
    }

    fn on_enter(&self, id: &Id, ctx: Context<'_, S>) {
        let span = ctx.span(id).expect("span must already exist!");
        if let Some(entry) = self.state.get_entry(span) {
            entry.track_entered();
        }
    }

    fn on_exit(&self, id: &Id, ctx: Context<'_, S>) {
        let span = ctx.span(id).expect("span must already exist!");
        if let Some(entry) = self.state.get_entry(span) {
            entry.track_exited();
        }
    }

    fn on_close(&self, id: Id, ctx: Context<'_, S>) {
        let span = ctx.span(&id).expect("span must already exist!");
        if let Some(entry) = self.state.get_entry(span) {
            entry.track_closed();
        }
    }

    unsafe fn downcast_raw(&self, id: TypeId) -> Option<*const ()> {
        match id {
            id if id == TypeId::of::<Self>() => Some(self as *const _ as *const ()),
            _ => None,
        }
    }
}
