//!
//! Yul to EasyCrypt translation
//!

pub mod arguments;
use self::arguments::Arguments;

use era_compiler_solidity::ECVisitor;
use era_compiler_solidity::{Project, Translator, WritePrinter, YulVisitor};

///
/// The application entry point.
///
fn main() {
    std::process::exit(match main_inner() {
        Ok(()) => era_compiler_common::EXIT_CODE_SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            era_compiler_common::EXIT_CODE_FAILURE
        }
    })
}

fn print_version() {
    println!(
        "Yul to EasyCrypt transpiler, part of {} v{}",
        env!("CARGO_PKG_DESCRIPTION"),
        env!("CARGO_PKG_VERSION"),
    );
}

///
/// The auxiliary `main` function to facilitate the `?` error conversion operator.
///
fn main_inner() -> anyhow::Result<()> {
    let arguments = Arguments::new();
    arguments.validate()?;

    if arguments.version {
        print_version();
        return Ok(());
    }

    let input_files = arguments.input_files_paths()?;
    let _suppressed_warnings = match arguments.suppress_warnings {
        Some(warnings) => Some(era_compiler_solidity::Warning::try_from_strings(
            warnings.as_slice(),
        )?),
        None => None,
    };

    let path = match input_files.len() {
        1 => input_files.first().expect("Always exists"),
        0 => anyhow::bail!("The input file is missing"),
        length => anyhow::bail!(
            "Only one input file is allowed in the Yul mode, but found {}",
            length,
        ),
    };

    let project = Project::try_from_yul_path(path, None)?;
    project.contracts.iter().for_each(|(_path, contr)| {
        if let Some(obj) = contr.get_yul_object() {
            WritePrinter::default().visit_object(obj);

            let mut t = Translator::new();
            let m = t.transpile_object(obj, true).unwrap();
            println!("{:#?}", m);

            WritePrinter::default().visit_module(&m);
        }
    });

    Ok(())
}