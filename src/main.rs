use std::{fs::File, path::{Path, PathBuf}};

use clap::Parser;
use faerie::{ArtifactBuilder, Link, Decl};
use target_lexicon::triple;
use iced_x86::code_asm::*;
use std::str::FromStr;

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

    let mut asm = CodeAssembler::new(64).unwrap();

    let mut label_main = asm.create_label();
    let mut label_add = asm.create_label();

    asm.set_label(&mut label_main).unwrap();
    asm.mov(rax, 69u64).unwrap();
    asm.call(label_add).unwrap();
    asm.ret().unwrap();

    asm.set_label(&mut label_add).unwrap();
    asm.add(rax, 10).unwrap();
    asm.ret().unwrap();

    let result = asm.assemble_options(0x0,4).unwrap();

    let main_ip = result.label_ip(&label_main).unwrap() as usize;
    let add_ip = result.label_ip(&label_add).unwrap() as usize;
    println!("main: {}, add: {}", main_ip, add_ip);

    let bytes = result.inner.code_buffer;
    let main = &bytes[main_ip..add_ip];
    let add = &bytes[add_ip..];

    let filename = "a.o";
    let f = File::create(Path::new(filename)).unwrap();
    let mut obj = ArtifactBuilder
        ::new(triple!("x86_64-unknown-unknown-unknown-macho"))
    .name(filename.to_owned())
    .finish();

    obj.declare_with("main", Decl::function().global(), main.to_owned())
        .unwrap();
    obj.declare_with("add", Decl::function().global(), add.to_owned())
        .unwrap();

    obj.write(f).unwrap();
}
