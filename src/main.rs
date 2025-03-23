use std::{fs::File, io::Read, path::{Path, PathBuf}};

use clap::Parser;
use codegen::{triple::get_triple, generate};
use preprocessor::preprocess;

pub mod preprocessor;
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

    /// Only run the preprocessor and send to STDOUT
    #[arg(short = 'E')]
    preprocess_only: bool
}

fn main() {
    let args = CLIArgs::parse();

    println!("{:?}", args);

    let file_contents = args.files.iter().map(|path| {
        let mut f = File::open(path).unwrap();
        let mut contents = String::new();
        f.read_to_string(&mut contents).unwrap();
        contents
    });

    let files_preproccessed = file_contents.map(|ref s| preprocess(s));

    if args.preprocess_only {
        for file in files_preproccessed {
            println!("{}", file);
        }

        return;
    }

    let triple = get_triple();

    println!("Compiling for {}", triple);

    generate(triple, Path::new("a.o"));
}
