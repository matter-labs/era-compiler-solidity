//!
//! A simple pretty printer that outputs text via a type implementing [`Write`]
//!
use std::io::{stdout, Write};

use super::IPrinter;

/// State of the printer
pub struct WritePrinter {
    indent: u32,
    line_start: bool,
    writer: Box<dyn Write>,
}

impl WritePrinter {
    const INDENT: &'static str = "  ";
    /// Creates a new [`Printer`].
    pub fn new(writer: Box<dyn Write>) -> WritePrinter {
        WritePrinter {
            indent: 0,
            line_start: true,
            writer,
        }
    }

    fn indent_reset(&mut self) {
        self.line_start = true;
    }
    fn indent(&mut self) {
        if self.line_start {
            for _ in 0..self.indent {
                let _ = self.writer.write_all(Self::INDENT.as_bytes());
            }
            self.line_start = false;
        }
    }
}

impl Default for WritePrinter {
    fn default() -> Self {
        Self::new(Box::new(stdout()))
    }
}

impl IPrinter for WritePrinter {
    fn print(&mut self, s: &str) {
        self.indent();
        let _ = self.writer.write_all(s.as_bytes());
    }

    fn println(&mut self, s: &str) {
        self.print(s);
        self.print("\n");
        self.indent_reset();
    }

    fn increase_indent(&mut self) {
        self.indent += 1
    }

    fn decrease_indent(&mut self) {
        if self.indent > 0 {
            self.indent -= 1
        }
    }
}
