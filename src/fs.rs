use std::{fs::File, io::Read, path::{Path, PathBuf}};

use log::trace;

use crate::error::CompilerError;

pub fn read_file(path: &Path) -> Result<String, CompilerError> {
    trace!("Reading file: {:?}", path);
    let mut f = File::open(path)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn replace_extension(mut path: PathBuf, extension: &str) -> PathBuf {
    // If there's not a file name we woulda found out by now lol
    let mut filename = path.file_stem().unwrap().to_os_string();
    filename.push(format!(".{}", extension));

    path.pop();
    path.push(filename);

    path
}

#[cfg(test)]
mod tests {
    use std::io::ErrorKind;

    use super::*;

    #[test]
    fn normal() -> Result<(), CompilerError> {
        let content = read_file(Path::new("tests/files/unit/fs/hello.txt"))?;
        assert_eq!("Hello World\n", content);
        Ok(())
    }

    #[test]
    fn not_found() -> Result<(), CompilerError> {
        let out = read_file(Path::new("tests/files/unit/fs/fake.txt"));

        match out {
            Ok(_) => panic!("Found file when it shouldn't exist!"),
            Err(e) => match e {
                CompilerError::FileError(f) => {
                    assert_eq!(f.kind(), ErrorKind::NotFound);
                    Ok(())
                },
                _ => Err(e)
            },
        }
    }
}
