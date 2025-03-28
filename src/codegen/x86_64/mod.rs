mod registers;
mod helpers;
mod instructions;
mod instance;
mod declaration;
mod statement;
mod expression;

use std::{fs::{rename, File}, io::Write, path::PathBuf, process::Command};

use instance::GeneratorInstance;
use log::{info, trace};
use tempfile::tempdir;
use crate::{ast::TranslationUnit, error::CompilerError, fs::replace_extension};
use super::{error::CodegenError, AssemblerOptions, Generator};

pub struct X86_64Generator;

impl X86_64Generator {
    pub fn new() -> X86_64Generator {
        X86_64Generator {}
    }
}

impl Generator for X86_64Generator {
    fn generate(&self, trans_unit: &TranslationUnit) -> 
        Result<String, CodegenError> {

        let mut instance = GeneratorInstance::new();

        for decl in &trans_unit.declarations {
            instance.gen_declaration(decl)?;
        }

        Ok(instance.get_instructions())
    }

    fn assemble(&self, 
        input_pairs: &[(PathBuf, String)],
        options: &AssemblerOptions
    ) -> Result<(), CompilerError> {
        
        let working_dir = tempdir()?;

        let mut asm_paths = vec![];
        for (path, asm) in input_pairs {
            let asm_path = replace_extension(working_dir.path().join(path), "s");

            let mut file = File::create(&asm_path)?;
            file.write_all(asm.as_bytes())?;

            asm_paths.push(asm_path);
        }

        let mut assembled = vec![];
        for path in asm_paths {
            // Invoke NASM
            let assembled_path = replace_extension(path.clone(), "o");

            trace!("Assembling {} to {}", path.display(), assembled_path.display());

            let nasm_output = Command::new("nasm")
                .arg("-f")
                .arg("macho64")
                .arg("-o")
                .arg(assembled_path.clone())
                .arg(path)
                .output()?;

            info!("NASM Output:\n{}", String::from_utf8(nasm_output.stderr).unwrap());

            assembled.push(assembled_path);
        }

        let pwd = std::env::current_dir()?;

        if !options.link {
            for assembled_file in &assembled {
                let filename = assembled_file.file_name().unwrap().to_owned();
                
                let filename_out = match (options.output, assembled.len()) {
                    (Some(o), 1) => o.to_owned(),
                    _ => pwd.join(filename),
                };

                rename(assembled_file, filename_out)?;
            }
        } else {
            // Invoke gcc to link into executable
            let gcc_output = Command::new("gcc")
                .arg("-target")
                .arg("x86_64-apple-darwin")
                .arg("-o")
                .arg(match options.output { 
                    Some(x) => x.to_str().unwrap(),
                    None => "a.out" 
                })
                .args(assembled)
                .output()?;

            info!("GCC Output:\n{}", String::from_utf8(gcc_output.stderr).unwrap());
        }


        Ok(())
    }
}
