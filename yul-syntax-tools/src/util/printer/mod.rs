//!
//! YUL pretty printer
//!
pub mod write_printer;

/// Interface to YUL pretty printer
pub trait IPrinter {
    /// Indent if in the beginning of line, and print string,
    fn print(&mut self, s: &str);
    /// Like [`print`], but adds a line feed
    fn println(&mut self, s: &str);
    /// Increase current indent
    fn increase_indent(&mut self);
    /// Decrease current indent
    fn decrease_indent(&mut self);
}

pub fn print_list_comma_separated<'a>(
    iter: impl IntoIterator<Item = &'a str>,
    printer: &mut impl IPrinter,
) {
    for (idx, a) in iter.into_iter().enumerate() {
        if idx > 0 {
            printer.print(", ")
        }
        printer.print(a)
    }
}
