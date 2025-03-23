pub fn preprocess(contents: &str) -> String {
    contents.lines().filter_map(|line| {
        if line.starts_with('#') {
            println!("Directive: {}", line);
            None
        } else {
            Some(line)
        }
    }).fold(String::new(), |mut acc, line| {
        acc.push_str(line);
        acc.push('\n');
        acc
    })
}
