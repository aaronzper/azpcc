use std::path::Path;

use indexmap::IndexMap;
use lalrpop_util::lalrpop_mod;

use crate::{error::CompilerError, fs::read_file};

mod directive;
use directive::Directive;

lalrpop_mod!(grammar, "/preprocessor/grammar.rs");

fn parse(file_contents: &str) -> Vec<Directive> {
    // First parse each line into either a raw string or the directive
    file_contents.lines().map(|line| {
        if line.starts_with('#') {
            grammar::DirectiveParser::new().parse(line).unwrap()
        } else {
            Directive::Raw(String::from(line))
        }
    // Then, combine any adjacent raw strings
    }).fold(Vec::new(), |mut directives, curr| {
        match directives.pop() {
            // We're the first directive, just push
            None => directives.push(curr),
            // Last directive was a raw, so combine us and them if we're
            // also a raw
            Some(Directive::Raw(line)) => {
                match curr {
                    // We're a raw! Combine and push
                    Directive::Raw(curr_line) => {
                        let combined = [line, curr_line].join("\n");
                        directives.push(Directive::Raw(combined));
                    }
                    // We're not :( so just push each
                    _ => {
                        directives.push(Directive::Raw(line));
                        directives.push(curr);
                    }
                }
            },
            // Last directive wasn't, so push it back alongside us
            Some(x) => {
                directives.push(x);
                directives.push(curr);
            }
        };

        directives
    })
}

fn get_directives(path: &Path) -> Result<Vec<Directive>, CompilerError> {
    let file_contents = read_file(path)?;
    let directives = parse(&file_contents);

    Ok(directives)
}

pub fn preprocess(path: &Path) -> Result<String, CompilerError> {
    let directives = get_directives(path)?;

    let mut definitions: IndexMap<String, Option<String>> = IndexMap::new();

    let mut output = String::new();
    for directive in directives {
        match directive {
            Directive::Raw(mut raw) => {
                for definition in &definitions {
                    let replace = match definition.1 {
                        Some(x) => &x,
                        None => "",
                    };

                    raw = raw.replace(definition.0, replace);
                }

                output.push_str(&raw);
            },

            Directive::Define(definition) => {
                if definitions.contains_key(&definition.identifier) {
                    definitions.shift_remove(&definition.identifier);
                }

                definitions.insert(
                    definition.identifier,
                    definition.replacement
                );
            },

            _ => return Err(CompilerError::NotSupported("#include Directives")),
        }
    }

    Ok(output)
}
