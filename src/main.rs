
use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let file_name = "large_file.txt";
    let num_lines = 500_000;
    let target_word = "Rust";
    let other_words = ["You guys are "];

    let content: String = (0..num_lines)
        .map(|i| {
            let word = if i % 100 == 0 {
                target_word
            } else {
                other_words[i % other_words.len()]
            };
            format!("{} are using best programming language.\n", word)
        })
        .collect();

    let mut file = File::create(file_name)?;
    write!(file, "{}", content)?;

    println!("Generated {} with {} lines", file_name, num_lines);

    Ok(())
}