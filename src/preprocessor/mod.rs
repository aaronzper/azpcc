use std::{collections::VecDeque, path::Path};

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

fn apply_definitions(
        definitions: &IndexMap<String, String>,
        input: &str
    ) -> String {

    let mut output = String::from(input);

    for definition in definitions {
        output = output.replace(definition.0, definition.1);
    }

    output
}

fn get_directives(path: &Path) -> Result<VecDeque<Directive>, CompilerError> {
    let file_contents = read_file(path)?;
    let directives = parse(&file_contents);

    // Use a VecDeque so we can add stuff to the front in the processing loop
    Ok(VecDeque::from(directives))
}

pub fn preprocess(path: &Path) -> Result<String, CompilerError> {
    let mut directives = get_directives(path)?;

    let mut definitions: IndexMap<String, String> = IndexMap::new();

    let mut output = String::new();

    while !directives.is_empty() {
        match directives.pop_front().unwrap() {

            Directive::Raw(raw) => {
                output.push_str(&apply_definitions(&definitions, &raw));
            },

            Directive::Define(definition) => {
                if definitions.contains_key(&definition.identifier) {
                    definitions.shift_remove(&definition.identifier);
                }

                let replace_with = match definition.replacement {
                    // Apply any existing defintions to this new one
                    Some(x) => apply_definitions(&definitions, &x),
                    None => String::new()
                };

                definitions.insert(definition.identifier, replace_with);
            },

            Directive::IncludeLocal(include_path) => {
                // Should always work. If we're a dir (e.g. no parent), we'll
                // get an error above somewhere
                let cwd = path.parent().unwrap();
                let full_path = cwd.join(include_path);
                let include_directives = get_directives(&full_path)?;

                for d in include_directives {
                    directives.push_front(d);
                };
            }

            // TODO: These
            _ => return Err(
                CompilerError::NotSupported("Global #include directives")
            ),
        }
    }

    Ok(output)
}
