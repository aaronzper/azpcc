use std::{collections::VecDeque, path::Path};

use indexmap::IndexMap;
use lalrpop_util::lalrpop_mod;
use log::{debug, trace};

use crate::{error::CompilerError, fs::read_file};

mod directive;
use directive::Directive;

lalrpop_mod!(grammar, "/preprocessor/grammar.rs");

fn parse(file_contents: &str) -> Result<Vec<Directive>, CompilerError> {
    // First parse each line into either a raw string or the directive
    file_contents.lines().map(|line| {
        if line.starts_with('#') {
            Ok(grammar::DirectiveParser::new().parse(line)?)
        } else {
            Ok(Directive::Raw(String::from(line)))
        }
    // Then, combine any adjacent raw strings
    }).fold(Ok(Vec::new()), |directives_res, curr_res| {
        let curr = match curr_res {
            Ok(x) => x,
            Err(e) => return Err(e),
        };

        let mut directives = match directives_res {
            Ok(x) => x,
            Err(e) => return Err(e),
        };

        match directives.pop() {
            // We're the first directive, just push
            None => directives.push(curr),
            // Last directive was a raw, so combine us and them if we're
            // also a raw
            Some(Directive::Raw(mut line)) => {
                match curr {
                    // We're a raw! Combine and push
                    Directive::Raw(curr_line) => {
                        let combined = [line, curr_line].join("\n");
                        directives.push(Directive::Raw(combined));
                    }
                    // We're not :(
                    _ => {
                        // Add \n to the end of each raw block
                        line.push('\n');
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

        Ok(directives)
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
    let directives = parse(&file_contents)?;

    // Use a VecDeque so we can add stuff to the front in the processing loop
    Ok(VecDeque::from(directives))
}

pub fn preprocess(path: &Path) -> Result<String, CompilerError> {
    let mut directives = get_directives(path)?;
    trace!("Produced directives: {:?}", directives);

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

                // Stick a \n at the end of the included directives so that the
                // #include replacement ends in a newline in the final file
                directives.push_front(Directive::Raw(String::from('\n')));
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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    fn path(ending: &str) -> PathBuf {
        let mut p = PathBuf::from("tests/files/unit/preproc/");
        p.push(ending);
        p
    }

    #[test]
    fn no_directives() -> Result<(), CompilerError> {
        let output = preprocess(&path("no_directives.txt"))?;
        assert_eq!(output, "Hello World\nTwo lines!!");
        Ok(())
    }

    #[test]
    fn defines() -> Result<(), CompilerError> {
        let output = preprocess(&path("defines.txt"))?;

        let expected = "hi
bye
hi bye
get redefined
1 + 2 + 3 >> 2
I like to say 1 + 2 + 3 >> 2";

        assert_eq!(output, expected);
        Ok(())
    }

    #[test]
    fn include_local() -> Result<(), CompilerError> {
        let output = preprocess(&path("include_local.txt"))?;
        let expected = "Hello from the header!
Hello from the file!
Hello from the header!
Yay!";

        assert_eq!(output, expected);
        Ok(())
    }

    #[test]
    fn include_define() -> Result<(), CompilerError> {
        let output = preprocess(&path("include_define.txt"))?;
        let expected = "HEADER\n\nhowdy";

        assert_eq!(output, expected);
        Ok(())
    }

    #[test]
    fn fake_directive() -> Result<(), CompilerError> {
        match preprocess(&path("fake_directive.txt")) {
            Ok(_) => panic!("Correctly parsed when we shouldn't have"),
            Err(e) => match e {
                CompilerError::ParseError(_) => Ok(()),
                _ => Err(e)
            }
        }
    }
}
