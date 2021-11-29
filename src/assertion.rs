use std::sync::Arc;

use crate::state::{EntryState, State};

pub struct Assertion {
    state: Arc<EntryState>,
}

impl Assertion {
    pub fn num_created(&self) -> usize {
		self.state.num_created()
	}

	pub fn was_created(&self) -> bool {
		self.num_created() > 0
	}

	pub fn num_entered(&self) -> usize {
		self.state.num_entered()
	}

	pub fn was_entered(&self) -> bool {
		self.num_entered() > 0
	}

	pub fn num_exited(&self) -> usize {
		self.state.num_exited()
	}

	pub fn was_exited(&self) -> bool {
		self.num_exited() > 0
	}

	pub fn num_closed(&self) -> usize {
		self.state.num_closed()
	}

	pub fn was_closed(&self) -> bool {
		self.num_closed() > 0
	}
}

pub struct Controller {
	state: Arc<State>,
}

impl Controller {
	pub fn new() -> Self {
		Self {
			state: Arc::new(State::default()),
		}
	}

	pub(crate) fn state(&self) -> &Arc<State> {
		&self.state
	}

	pub fn add_entry<S>(&self, name: S) -> Assertion
	where
		S: AsRef<str>,
	{
		let name = name.as_ref().to_string();
		let state = self.state.create_entry(name);

		Assertion { state }
	}
}