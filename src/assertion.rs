//! Core assertion types and utilities.
use std::{marker::PhantomData, sync::Arc};

use crate::{
    matcher::SpanMatcher,
    state::{EntryState, State},
};

enum AssertionCriterion {
    WasCreated,
    WasEntered,
    WasExited,
    WasClosed,
    WasNotCreated,
    WasNotEntered,
    WasNotExited,
    WasNotClosed,
    CreatedExactly(usize),
    EnteredExactly(usize),
    ExitedExactly(usize),
    ClosedExactly(usize),
    CreatedAtLeast(usize),
    EnteredAtLeast(usize),
    ExitedAtLeast(usize),
    ClosedAtLeast(usize),
}

impl AssertionCriterion {
    pub fn assert(&self, state: &Arc<EntryState>) {
        match self {
            AssertionCriterion::WasCreated => assert!(state.num_created() != 0),
            AssertionCriterion::WasEntered => assert!(state.num_entered() != 0),
            AssertionCriterion::WasExited => assert!(state.num_exited() != 0),
            AssertionCriterion::WasClosed => assert!(state.num_closed() != 0),
            AssertionCriterion::WasNotCreated => assert_eq!(0, state.num_created()),
            AssertionCriterion::WasNotEntered => assert_eq!(0, state.num_entered()),
            AssertionCriterion::WasNotExited => assert_eq!(0, state.num_exited()),
            AssertionCriterion::WasNotClosed => assert_eq!(0, state.num_closed()),
            AssertionCriterion::CreatedExactly(times) => assert_eq!(state.num_created(), *times),
            AssertionCriterion::EnteredExactly(times) => assert_eq!(state.num_entered(), *times),
            AssertionCriterion::ExitedExactly(times) => assert_eq!(state.num_exited(), *times),
            AssertionCriterion::ClosedExactly(times) => assert_eq!(state.num_closed(), *times),
            AssertionCriterion::CreatedAtLeast(times) => assert!(state.num_created() >= *times),
            AssertionCriterion::EnteredAtLeast(times) => assert!(state.num_entered() >= *times),
            AssertionCriterion::ExitedAtLeast(times) => assert!(state.num_exited() >= *times),
            AssertionCriterion::ClosedAtLeast(times) => assert!(state.num_closed() >= *times),
        }
    }

    pub fn try_assert(&self, state: &Arc<EntryState>) -> bool {
        match self {
            AssertionCriterion::WasCreated => state.num_created() != 0,
            AssertionCriterion::WasEntered => state.num_entered() != 0,
            AssertionCriterion::WasExited => state.num_exited() != 0,
            AssertionCriterion::WasClosed => state.num_closed() != 0,
            AssertionCriterion::WasNotCreated => state.num_created() == 0,
            AssertionCriterion::WasNotEntered => state.num_entered() == 0,
            AssertionCriterion::WasNotExited => state.num_exited() == 0,
            AssertionCriterion::WasNotClosed => state.num_closed() == 0,
            AssertionCriterion::CreatedExactly(times) => state.num_created() == *times,
            AssertionCriterion::EnteredExactly(times) => state.num_entered() == *times,
            AssertionCriterion::ExitedExactly(times) => state.num_exited() == *times,
            AssertionCriterion::ClosedExactly(times) => state.num_closed() == *times,
            AssertionCriterion::CreatedAtLeast(times) => state.num_created() >= *times,
            AssertionCriterion::EnteredAtLeast(times) => state.num_entered() >= *times,
            AssertionCriterion::ExitedAtLeast(times) => state.num_exited() >= *times,
            AssertionCriterion::ClosedAtLeast(times) => state.num_closed() >= *times,
        }
    }
}

/// A specific set of criteria to enforce on matching spans.
///
/// Assertions represent both a span "matcher" -- which controls which spans the criteria are
/// applied to -- and the criteria themselves, which define the behavior to assert against the
/// matching spans.
///
/// ## Matching behavior
///
/// As an `Assertion` can match multiple spans, care must be taken when building the `Assertion` to
/// constrain the matcher correctly.  For example, if you want to focus on a specific span, you
/// would want to use match on the span name at a minimum, and potentially match on the span target
/// if there were other spans with the same name in different modules.  However, if you simply
/// wanted to check if any spans under a specific module path were created -- perhaps to assert that
/// a particular codeflow is being exercised, but not assert _how_ it's being exercised -- then only
/// specifying the span target might suffice.
pub struct Assertion {
    state: Arc<State>,
    entry_state: Arc<EntryState>,
    matcher: SpanMatcher,
    criteria: Vec<AssertionCriterion>,
}

impl Assertion {
    /// Asserts that all criteria have been met.
    ///
    /// Uses the "assert" macros from the standard library, so criterion which have not been met
    /// will cause a panic, similar to using the "assert" macros directly.
    ///
    /// For a fallible assertion that can be called over and over without panicking, [`try_assert`]
    /// can be used instead.
    pub fn assert(&self) {
        for criterion in &self.criteria {
            criterion.assert(&self.entry_state);
        }
    }

    /// Attempts to assert that all criteria have been met.
    ///
    /// If any of the criteria have not yet been met, `false` will be returned.  Otherwise, `true`
    /// will be returned.
    ///
    /// If assertions should end your test immediately, [`assert`] can be used instead.
    pub fn try_assert(&self) -> bool {
        for criterion in &self.criteria {
            if !criterion.try_assert(&self.entry_state) {
                return false;
            }
        }

        true
    }
}

impl Drop for Assertion {
    fn drop(&mut self) {
        self.state.remove_entry(&self.matcher);
    }
}

/// An [`AssertionBuilder`] which does not yet have a span matcher.
///
/// A matcher consists of either a span name, or the target of a span itself, or potentially both.
/// A span target refers to the `tracing` parlance, where "target" refers to the module path that a
/// span is defined in.
///
/// Additionally, a span matcher can include specific fields that must be present on a span in order
/// to match.
pub struct NoMatcher {
    _p: PhantomData<()>,
}

/// An [`AssertionBuilder`] which has a valid span matcher but does not yet have any assertion
/// criteria.
///
/// Assertion criteria are the actual behavioral matchers, such as "this span must have been entered
/// at least once" or "this span must have been created at least N times".
pub struct NoCriteria {
    _p: PhantomData<()>,
}

/// An [`AssertionBuilder`] which has a valid span matcher and at least one assertion criterion.
pub struct Constrained {
    _p: PhantomData<()>,
}

/// Configures and constructs an [`Assertion`].
///
/// This builder uses a state pattern to ensure that the necessary fields are configured before a
/// valid `Assertion` can be constructed.  You may notice that some methods are only available once
/// other methods have been called.
///
/// You must first define a span matcher, which defines how this assertion is matched to a given
/// span, and then you must specify the assertion criteria itself, which defines the behavior of the
/// span to assert for.
///
/// Once these are defined, an `Assertion` can be constructed by calling [`finalize`].
pub struct AssertionBuilder<S> {
    state: Arc<State>,
    matcher: Option<SpanMatcher>,
    criteria: Vec<AssertionCriterion>,
    _builder_state: PhantomData<fn(S)>,
}

impl AssertionBuilder<NoMatcher> {
    /// Sets the name of the span to match.
    ///
    /// All span matchers, which includes [`with_name`], [`with_target`], [`with_parent_name`], and
    /// [`with_span_field`], are additive, which means a span must match all of them to match the
    /// assertion overall.
    pub fn with_name<S>(mut self, name: S) -> AssertionBuilder<NoCriteria>
    where
        S: Into<String>,
    {
        let matcher = self.matcher.get_or_insert_with(SpanMatcher::default);
        matcher.set_name(name.into());

        AssertionBuilder {
            state: self.state,
            matcher: self.matcher,
            criteria: self.criteria,
            _builder_state: PhantomData,
        }
    }

    /// Sets the target of the span to match.
    ///
    /// All span matchers, which includes [`with_name`], [`with_target`], [`with_parent_name`], and
    /// [`with_span_field`], are additive, which means a span must match all of them to match the
    /// assertion overall.
    pub fn with_target<S>(mut self, target: S) -> AssertionBuilder<NoCriteria>
    where
        S: Into<String>,
    {
        let matcher = self.matcher.get_or_insert_with(SpanMatcher::default);
        matcher.set_target(target.into());

        AssertionBuilder {
            state: self.state,
            matcher: self.matcher,
            criteria: self.criteria,
            _builder_state: PhantomData,
        }
    }
}

impl AssertionBuilder<NoCriteria> {
    /// Sets the name of the span to match.
    ///
    /// All span matchers, which includes [`with_name`], [`with_target`], [`with_parent_name`], and
    /// [`with_span_field`], are additive, which means a span must match all of them to match the
    /// assertion overall.
    pub fn with_name<S>(mut self, name: S) -> AssertionBuilder<NoCriteria>
    where
        S: Into<String>,
    {
        let matcher = self.matcher.get_or_insert_with(SpanMatcher::default);
        matcher.set_name(name.into());

        AssertionBuilder {
            state: self.state,
            matcher: self.matcher,
            criteria: self.criteria,
            _builder_state: PhantomData,
        }
    }

    /// Sets the target of the span to match.
    ///
    /// All span matchers, which includes [`with_name`], [`with_target`], [`with_parent_name`], and
    /// [`with_span_field`], are additive, which means a span must match all of them to match the
    /// assertion overall.
    pub fn with_target<S>(mut self, target: S) -> AssertionBuilder<NoCriteria>
    where
        S: Into<String>,
    {
        let matcher = self.matcher.get_or_insert_with(SpanMatcher::default);
        matcher.set_target(target.into());

        AssertionBuilder {
            state: self.state,
            matcher: self.matcher,
            criteria: self.criteria,
            _builder_state: PhantomData,
        }
    }

    /// Sets the name of a parent span to match.
    ///
    /// The span must have at least one parent span within its entire lineage that matches the given
    /// name.
    ///
    /// All span matchers, which includes [`with_name`], [`with_target`], [`with_parent_name`], and
    /// [`with_span_field`], are additive, which means a span must match all of them to match the
    /// assertion overall.
    pub fn with_parent_name<S>(mut self, name: S) -> AssertionBuilder<NoCriteria>
    where
        S: Into<String>,
    {
        let matcher = self.matcher.get_or_insert_with(SpanMatcher::default);
        matcher.set_parent_name(name.into());

        AssertionBuilder {
            state: self.state,
            matcher: self.matcher,
            criteria: self.criteria,
            _builder_state: PhantomData,
        }
    }

    /// Adds a field which the span must contain to match.
    ///
    /// The field is matched by name.
    ///
    /// All span matchers, which includes [`with_name`], [`with_target`], and [`with_span_field`],
    /// are additive, which means a span must match all of them to match the assertion overall.
    pub fn with_span_field<S>(mut self, field: S) -> AssertionBuilder<NoCriteria>
    where
        S: Into<String>,
    {
        if let Some(matcher) = self.matcher.as_mut() {
            matcher.add_field_exists(field.into());
        }

        AssertionBuilder {
            state: self.state,
            matcher: self.matcher,
            criteria: self.criteria,
            _builder_state: PhantomData,
        }
    }

    /// Asserts that a matching span was created at least once.
    pub fn was_created(mut self) -> AssertionBuilder<Constrained> {
        self.criteria.push(AssertionCriterion::WasCreated);

        AssertionBuilder {
            state: self.state,
            matcher: self.matcher,
            criteria: self.criteria,
            _builder_state: PhantomData,
        }
    }

    /// Asserts that a matching span was entered at least once.
    pub fn was_entered(mut self) -> AssertionBuilder<Constrained> {
        self.criteria.push(AssertionCriterion::WasEntered);

        AssertionBuilder {
            state: self.state,
            matcher: self.matcher,
            criteria: self.criteria,
            _builder_state: PhantomData,
        }
    }

    /// Asserts that a matching span was exited at least once.
    pub fn was_exited(mut self) -> AssertionBuilder<Constrained> {
        self.criteria.push(AssertionCriterion::WasExited);

        AssertionBuilder {
            state: self.state,
            matcher: self.matcher,
            criteria: self.criteria,
            _builder_state: PhantomData,
        }
    }

    /// Asserts that a matching span was closed at least once.
    pub fn was_closed(mut self) -> AssertionBuilder<Constrained> {
        self.criteria.push(AssertionCriterion::WasClosed);

        AssertionBuilder {
            state: self.state,
            matcher: self.matcher,
            criteria: self.criteria,
            _builder_state: PhantomData,
        }
    }

    /// Asserts that a matching span was not created.
    pub fn was_not_created(mut self) -> AssertionBuilder<Constrained> {
        self.criteria.push(AssertionCriterion::WasNotCreated);

        AssertionBuilder {
            state: self.state,
            matcher: self.matcher,
            criteria: self.criteria,
            _builder_state: PhantomData,
        }
    }

    /// Asserts that a matching span was not entered.
    pub fn was_not_entered(mut self) -> AssertionBuilder<Constrained> {
        self.criteria.push(AssertionCriterion::WasNotEntered);

        AssertionBuilder {
            state: self.state,
            matcher: self.matcher,
            criteria: self.criteria,
            _builder_state: PhantomData,
        }
    }

    /// Asserts that a matching span was not exited.
    pub fn was_not_exited(mut self) -> AssertionBuilder<Constrained> {
        self.criteria.push(AssertionCriterion::WasNotExited);

        AssertionBuilder {
            state: self.state,
            matcher: self.matcher,
            criteria: self.criteria,
            _builder_state: PhantomData,
        }
    }

    /// Asserts that a matching span was not closed.
    pub fn was_not_closed(mut self) -> AssertionBuilder<Constrained> {
        self.criteria.push(AssertionCriterion::WasNotClosed);

        AssertionBuilder {
            state: self.state,
            matcher: self.matcher,
            criteria: self.criteria,
            _builder_state: PhantomData,
        }
    }

    /// Asserts that a matching span was created exactly `n` times.
    pub fn was_created_exactly(mut self, n: usize) -> AssertionBuilder<Constrained> {
        self.criteria.push(AssertionCriterion::CreatedExactly(n));

        AssertionBuilder {
            state: self.state,
            matcher: self.matcher,
            criteria: self.criteria,
            _builder_state: PhantomData,
        }
    }

    /// Asserts that a matching span was entered exactly `n` times.
    pub fn was_entered_exactly(mut self, n: usize) -> AssertionBuilder<Constrained> {
        self.criteria.push(AssertionCriterion::EnteredExactly(n));

        AssertionBuilder {
            state: self.state,
            matcher: self.matcher,
            criteria: self.criteria,
            _builder_state: PhantomData,
        }
    }

    /// Asserts that a matching span was exited exactly `n` times.
    pub fn was_exited_exactly(mut self, n: usize) -> AssertionBuilder<Constrained> {
        self.criteria.push(AssertionCriterion::ExitedExactly(n));

        AssertionBuilder {
            state: self.state,
            matcher: self.matcher,
            criteria: self.criteria,
            _builder_state: PhantomData,
        }
    }

    /// Asserts that a matching span was closed exactly `n` times.
    pub fn was_closed_exactly(mut self, n: usize) -> AssertionBuilder<Constrained> {
        self.criteria.push(AssertionCriterion::ClosedExactly(n));

        AssertionBuilder {
            state: self.state,
            matcher: self.matcher,
            criteria: self.criteria,
            _builder_state: PhantomData,
        }
    }

    /// Asserts that a matching span was created at least `n` times.
    pub fn was_created_at_least(mut self, n: usize) -> AssertionBuilder<Constrained> {
        self.criteria.push(AssertionCriterion::CreatedAtLeast(n));

        AssertionBuilder {
            state: self.state,
            matcher: self.matcher,
            criteria: self.criteria,
            _builder_state: PhantomData,
        }
    }

    /// Asserts that a matching span was entered at least `n` times.
    pub fn was_entered_at_least(mut self, n: usize) -> AssertionBuilder<Constrained> {
        self.criteria.push(AssertionCriterion::EnteredAtLeast(n));

        AssertionBuilder {
            state: self.state,
            matcher: self.matcher,
            criteria: self.criteria,
            _builder_state: PhantomData,
        }
    }

    /// Asserts that a matching span was exited at least `n` times.
    pub fn was_exited_at_least(mut self, n: usize) -> AssertionBuilder<Constrained> {
        self.criteria.push(AssertionCriterion::ExitedAtLeast(n));

        AssertionBuilder {
            state: self.state,
            matcher: self.matcher,
            criteria: self.criteria,
            _builder_state: PhantomData,
        }
    }

    /// Asserts that a matching span was closed at least `n` times.
    pub fn was_closed_at_least(mut self, n: usize) -> AssertionBuilder<Constrained> {
        self.criteria.push(AssertionCriterion::ClosedAtLeast(n));

        AssertionBuilder {
            state: self.state,
            matcher: self.matcher,
            criteria: self.criteria,
            _builder_state: PhantomData,
        }
    }
}

impl AssertionBuilder<Constrained> {
    /// Asserts that a matching span was created at least once.
    pub fn was_created(mut self) -> Self {
        self.criteria.push(AssertionCriterion::WasCreated);
        self
    }

    /// Asserts that a matching span was entered at least once.
    pub fn was_entered(mut self) -> Self {
        self.criteria.push(AssertionCriterion::WasEntered);
        self
    }

    /// Asserts that a matching span was exited at least once.
    pub fn was_exited(mut self) -> Self {
        self.criteria.push(AssertionCriterion::WasExited);
        self
    }

    /// Asserts that a matching span was closed at least once.
    pub fn was_closed(mut self) -> Self {
        self.criteria.push(AssertionCriterion::WasClosed);
        self
    }

    /// Asserts that a matching span was not created.
    pub fn was_not_created(mut self) -> Self {
        self.criteria.push(AssertionCriterion::WasNotCreated);
        self
    }

    /// Asserts that a matching span was not entered.
    pub fn was_not_entered(mut self) -> Self {
        self.criteria.push(AssertionCriterion::WasNotEntered);
        self
    }

    /// Asserts that a matching span was not exited.
    pub fn was_not_exited(mut self) -> Self {
        self.criteria.push(AssertionCriterion::WasNotExited);
        self
    }

    /// Asserts that a matching span was not closed.
    pub fn was_not_closed(mut self) -> Self {
        self.criteria.push(AssertionCriterion::WasNotClosed);
        self
    }

    /// Asserts that a matching span was created exactly `n` times.
    pub fn was_created_exactly(mut self, n: usize) -> Self {
        self.criteria.push(AssertionCriterion::CreatedExactly(n));
        self
    }

    /// Asserts that a matching span was entered exactly `n` times.
    pub fn was_entered_exactly(mut self, n: usize) -> Self {
        self.criteria.push(AssertionCriterion::EnteredExactly(n));
        self
    }

    /// Asserts that a matching span was exited exactly `n` times.
    pub fn was_exited_exactly(mut self, n: usize) -> Self {
        self.criteria.push(AssertionCriterion::ExitedExactly(n));
        self
    }

    /// Asserts that a matching span was closed exactly `n` times.
    pub fn was_closed_exactly(mut self, n: usize) -> Self {
        self.criteria.push(AssertionCriterion::ClosedExactly(n));
        self
    }

    /// Asserts that a matching span was created at least `n` times.
    pub fn was_created_at_least(mut self, n: usize) -> Self {
        self.criteria.push(AssertionCriterion::CreatedAtLeast(n));
        self
    }

    /// Asserts that a matching span was entered at least `n` times.
    pub fn was_entered_at_least(mut self, n: usize) -> Self {
        self.criteria.push(AssertionCriterion::EnteredAtLeast(n));
        self
    }

    /// Asserts that a matching span was exited at least `n` times.
    pub fn was_exited_at_least(mut self, n: usize) -> Self {
        self.criteria.push(AssertionCriterion::ExitedAtLeast(n));
        self
    }

    /// Asserts that a matching span was closed at least `n` times.
    pub fn was_closed_at_least(mut self, n: usize) -> Self {
        self.criteria.push(AssertionCriterion::ClosedAtLeast(n));
        self
    }

    /// Creates the finalized `Assertion`.
    ///
    /// Once finalized, the assertion is live and its state will be updated going forward.
    pub fn finalize(mut self) -> Assertion {
        let matcher = self
            .matcher
            .take()
            .expect("matcher must be present at this point");
        let entry_state = self.state.create_entry(matcher.clone());
        Assertion {
            state: Arc::clone(&self.state),
            entry_state,
            matcher,
            criteria: self.criteria,
        }
    }
}

/// Creates and stores all constructed [`Assertion`]s.
#[derive(Clone, Default)]
pub struct AssertionRegistry {
    state: Arc<State>,
}

impl AssertionRegistry {
    pub(crate) fn state(&self) -> &Arc<State> {
        &self.state
    }

    /// Creates an [`AssertionBuilder`] for constructing a new [`Assertion`].
    pub fn build(&self) -> AssertionBuilder<NoMatcher> {
        AssertionBuilder {
            state: Arc::clone(&self.state),
            matcher: None,
            criteria: Vec::new(),
            _builder_state: PhantomData,
        }
    }
}
