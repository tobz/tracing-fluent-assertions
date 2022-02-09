# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->

## [Unreleased] - ReleaseDate

## [0.3.0] - 2022-02-09

### Changed

- Updated to `tracing-subscriber@v0.3`.

### Fixed

- Dropping an `Assertion` now removes the backing state used to collect the different span events,
  which means that tests which run the same code over multiple test cases won't accumulate data in
  the assertion state over time that would lead to inconsistency.  Said differently, creating an
  assertion for a second time, so long as the first has been dropped, will correctly start out with
  a clean state now.

## [0.2.0] - 2021-12-16

### Added

- New assertion family: `was_not_*`.  Now you can assert that a given lifecycle event did not occur
  i.e. a span was not entered, or hasn't yet closed, etc.
- New assertion family: `was_*_exactly`.  These assertions replace `was_*_many`, which were named as
  if they could potentially be either "at least" or "exactly", while the docs said explicitly that
  they were "at least", but in reality, the checks were "exactly."  Now we have each group
  respectively with better naming.

### Changed

- Renamed the `was_*_many` assertion family to `was_*_at_least` to better reflect their "at least"
  nature, and to match the naming scheme of the newly-added `was_*_exactly` assertion family.

## [0.1.3] - 2021-11-30

### Added

- `AssertionRegistry` can now be cloned.

## [0.1.2] - 2021-11-29

### Added

- Ability to require a parent span (any parent, not direct predecessor) to match a specific name.

## [0.1.1] - 2021-11-29

### Changed

- Tweaks to documentation.  No code changes.

## [0.1.0] - 2021-11-29

### Added

- Genesis of the crate.
