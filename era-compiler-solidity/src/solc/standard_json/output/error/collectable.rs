//!
//! The collectable errors trait.
//!

use std::io::Write;

use crate::solc::standard_json::output::error::Error;

///
/// The collectable errors trait.
///
/// Should be implemented by entities that can collect errors, and perform actions
/// on them as a list upon request.
///
pub trait Collectable {
    ///
    /// Returns errors as a list.
    ///
    fn errors(&self) -> Vec<&Error>;

    ///
    /// Returns warnings as a list.
    ///
    fn warnings(&self) -> Vec<&Error>;

    ///
    /// Removes warnings from the list of messages.
    ///
    fn remove_warnings(&mut self);

    ///
    /// Checks if there is at least one error.
    ///
    fn has_errors(&self) -> bool {
        !self.errors().is_empty()
    }

    ///
    /// Checks if there is at least one warning.
    ///
    fn has_warnings(&self) -> bool {
        !self.warnings().is_empty()
    }

    ///
    /// Collects errors into one message and bails, if there is at least one error.
    ///
    fn collect_errors(&self) -> anyhow::Result<()> {
        if !self.has_errors() {
            return Ok(());
        }

        anyhow::bail!(
            "{}",
            self.errors()
                .iter()
                .map(|error| error.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        );
    }

    ///
    /// Checks for errors, exiting the application if there is at least one error.
    ///
    fn exit_on_error(&self) {
        if !self.has_errors() {
            return;
        }

        std::io::stderr()
            .write_all(
                self.errors()
                    .iter()
                    .map(|error| error.to_string())
                    .collect::<Vec<String>>()
                    .join("\n")
                    .as_bytes(),
            )
            .expect("Stderr writing error");
        std::process::exit(era_compiler_common::EXIT_CODE_FAILURE);
    }

    ///
    /// Removes warnings from the list of messages and prints them to stderr.
    ///
    fn take_and_write_warnings(&mut self) {
        if !self.has_warnings() {
            return;
        }

        writeln!(
            std::io::stderr(),
            "{}",
            self.warnings()
                .iter()
                .map(|error| error.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        )
        .expect("Stderr writing error");

        self.remove_warnings();
    }
}
