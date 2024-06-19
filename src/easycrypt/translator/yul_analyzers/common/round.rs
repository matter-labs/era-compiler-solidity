//!
//! A single round of inference.
//!

/// A single round of inference. Aimed at recording an effect so that the pass
/// knows when it reaches a fixpoint.
#[derive(Default)]
pub struct Round {
    had_effect: bool,
}

impl Round {
    /// Returns a new instance.
    pub fn new() -> Self {
        Default::default()
    }

    /// Returns true if the pass has made changes in state.
    pub fn had_effect(&self) -> bool {
        self.had_effect
    }

    /// Mark that the pass has made changes in state.
    pub fn register_effect(&mut self) {
        self.had_effect = true
    }
}
