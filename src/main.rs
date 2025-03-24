use std::path::{Path, PathBuf};

use ast::{SemanticUnit, TranslationUnit};
use clap::{Parser, ValueEnum};
use codegen::{triple::get_triple, generate};
use colog::basic_builder;
use error::CompilerError;
use log::{debug, info, error, LevelFilter};
use parser::parse;
use preprocessor::preprocess;

pub mod error;
pub mod fs;
pub mod preprocessor;
pub mod ast;
pub mod parser;
pub mod codegen;

#[derive(Debug, Clone, ValueEnum)]
enum LogLevel {
    /// Default setting, outputs errors and warnings only
    Standard,
    /// Outputs informational logs, in addition to the above
    Info,
    /// Outputs the above, plus debugging logs
    Debug,
    /// Outputs the above, plus low-level tracing logs
    Trace,
}

impl From<&LogLevel> for LevelFilter {
    fn from(value: &LogLevel) -> Self {
        match value {
            LogLevel::Standard => Self::Warn,
            LogLevel::Info => Self::Info,
            LogLevel::Debug => Self::Debug,
            LogLevel::Trace => Self::Trace,
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about)]
struct CLIArgs {
    /// The .c files to compile
    #[arg(required=true, num_args=1..)]
    files: Vec<PathBuf>,

    /// Compile without linking to an executable
    #[arg(short)]
    compile_only: bool,

    /// Output file path
    #[arg(short, value_name = "PATH")]
    output: Option<PathBuf>,

    /* TODO: This
    /// Define a macro for the preprocessor
    #[arg(short = 'D', value_name = "MACRO")]
    defines: Vec<String>,

    /// Include the given directory in the include search path
    #[arg(short = 'I', value_name = "PATH")]
    includes: Vec<PathBuf>,
    */

    /// Only run the preprocessor and send to STDOUT
    #[arg(short = 'E')]
    preprocess_only: bool,

    /// Don't run the assembler; instead send emitted assembly to STDOUT
    #[arg(short = 'S')]
    emit_assembly: bool,

    /// Print the AST generated by the parser
    #[arg(long)]
    ast: bool,

    /// How detailed should logs be?
    #[arg(short, long, default_value_t = LogLevel::Standard, value_enum)]
    log_level: LogLevel,
}

fn entry() -> Result<(), CompilerError> {
    let args = CLIArgs::parse();

    let mut log_builder = basic_builder();
    log_builder.filter_level(LevelFilter::from(&args.log_level));
    log_builder.init();

    debug!("Starting compiler withs args {:?}", args);

    info!("Starting preprocessing");
    let files_preproccessed = args.files.iter().map(|ref s| preprocess(s));

    if args.preprocess_only {
        for file in files_preproccessed {
            println!("{}", file?);
        }

        return Ok(());
    }

    info!("Starting parsing");
    let files_parsed = files_preproccessed.map(|s| match s {
        Ok(x) => {
            let parsed = parse(&x)?;
            parsed.verify()?;
            Ok(parsed)
        },
        Err(e) => Err(e),
    });

    if args.ast {
        for file in files_parsed {
            println!("{:#?}", file?);
        }

        return Ok(());
    }

    let triple = get_triple();

    info!("Starting code generation for {}", triple);

    generate(triple, Path::new("a.o"))
}

fn main() {
    match entry() {
        Ok(_) => (),
        Err(e) => error!("{}", e)
    }
}
