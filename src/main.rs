use std::{fs::File, path::{Path, PathBuf}};

use clap::Parser;
use codegen::x86_64::generate;

pub mod codegen;

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

    /// Verbose logging output
    #[arg(short, long)]
    verbose: bool,

    /// Define a macro for the preprocessor
    #[arg(short = 'D', value_name = "MACRO")]
    defines: Vec<String>,

    /// Include the given directory in the include search path
    #[arg(short = 'I', value_name = "PATH")]
    includes: Vec<PathBuf>,
}

fn main() {
    let args = CLIArgs::parse();

    println!("{:?}", args);





    let triple = crate::codegen::triple::get_triple();

    println!("Compiling for {}", triple);

    generate(triple, Path::new("a.o"));
}
