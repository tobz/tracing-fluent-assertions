# tracing-fluent-assertions
A fluent assertions framework for [`tracing`](https://docs.rs/tracing).

## overview

While there are already many crates that deal with testing -- mocks, test doubles, advanced
assertions, etc -- there aren't any crates that allow a user to understand how their
[`tracing`](https://docs.rs/tracing) implementation is exercised at a holistic level.  While there
are some crates, like [`tracing-test`](https://docs.rs/tracing-test), which exist for figuring out
if a chunk of code emitted certain events, there is no generic way to ask questions like:

- was span A ever created? or entered?
- did it ever close?
- did it enter/exit/close at least N times?
- did any spans in module path X ever get created?

This is the problem that `tracing-fluent-assertions` aims to solve.

## usage

This crate doesn't look terribly dissimilar to other crates which provide fluent assertions, but as
it's oriented around spans, which are callsite-defined, there's a little bit of boilerplate involved
in using it compared to defining assertions directly against the result of a function, and so on.

Firstly, it provides a [`Subscriber`](https://docs.rs/tracing/latest/tracing/trait.Subscriber.html)
layer that must be installed so that it can intercept span events and track the lifecycle of spans.
Secondly, an
[`AssertionRegistry`](https://docs.rs/tracing-fluent-assertions/latest/tracing_fluent_assertions/assertion/struct.AssertionRegistry.html)
is provided for creating and storing assertions.

An
[`Assertion`](https://docs.rs/tracing-fluent-assertions/latest/tracing_fluent_assertions/assertion/struct.Assertion.html
) defines what spans it should match, and what behavior the spans must match in order to assert
successfully.

A condensed usage might look something like this:

```rust
use tracing_fluent_assertions::{AssertionLayer, AssertionRegistry};
use tracing_subscriber::{layer::SubscriberExt, Registry};

fn main() {
    // Create the assertion registry and install the assertion layer,
    // then install that subscriber as the global default.
    let assertion_registry = AssertionRegistry::default();
    let base_subscriber = Registry::default();
    let subscriber = base_subscriber.with(AssertionsLayer::new(&assertion_registry));
    tracing::subscriber::set_global_default(subscriber).unwrap();

    // Create an assertion.  We'll look for a span called `shave_yak`,
    // and assert that it's closed at least twice, signalling two full
    // create/enter/exit/closed instances of the span.  Essentially, at
    // least two yaks were completely shaved.
    let more_than_one_shaved_yak = assertion_registry.build()
        .with_name("shave_yak")
        .was_closed_many(2)
        .finalize();

    // Now, call our method that actually shaves the yaks.
    shave_yaks(5);

    // Assuming all five yaks were shaved, this assertion will pass,
    // and no panic will be generated, yay!
    more_than_one_yak_shaved.assert();

    // An advanced usage of assertions can be to figure out when a span
    // has finally been entered.  This can be useful for ascertaining when
    // an asynchronous function has made it through other await points and
    // now waiting at a piece of code that we control, with its own span.
    //
    // For this, we can use the fallible `try_assert`, which won't panic
    // if the assertion criteria has yet to be entirely met:
    let reached_acquire_shaving_shears = assertion_registry.build()
        .with_name("acquire_shaving_shears")
        .was_entered()
        .finalize();

    let manual_future = shave_yaks_async(5);

    assert!(!reached_acquire_shaving_shears.try_assert());
    while !reached_acquire_shaving_shears.try_assert() {
        manual_future.poll();
    }

    // Once we break out of that loop, we know that we have entered the
    // acquire_shaving_shears` span at least once.  This example is a bit
    // contrived, but a more useful scenario (albeit with more code required
    // to demonstrate) would be to figure out that one asynchronous task is
    // finally awaiting a specific resource, when it has to await other resources
    // that can't be deterministically controlled when under test.
}
```
