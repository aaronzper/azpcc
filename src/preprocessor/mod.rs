pub fn preprocess(contents: &str) -> String {
    let lines = contents.lines();

    let output = lines.filter_map(|line| {
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
    });

    output
}
