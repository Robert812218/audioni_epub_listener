use std::fs;
use std::io::{BufReader, Read};
use std::path::Path;

// Define a struct to hold the title and content of a text file
struct TextFile {
    title: String,
    content: String,
}

// Define a function to read the contents of a file given its path
fn read_file(path: &Path) -> Result<String, std::io::Error> {
    let file = fs::File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content)?;
    Ok(content)
}

fn main() {
    let folder_path = Path::new("src/library");
    let mut text_files = Vec::new();

    for entry in fs::read_dir(folder_path).unwrap() {
        let entry = entry.unwrap();
        let file_path = entry.path();
        let file_name = entry.file_name().into_string().unwrap();

        if file_path.is_file() {
            if let Ok(content) = read_file(&file_path) {
                let text_file = TextFile {
                    title: file_name,
                    content: content,
                };
                text_files.push(text_file);
            }
        }
    }

    // Do something with the text_files vector
    for text_file in text_files {
        println!("Title: {}", text_file.title);
        println!("Content: {}", text_file.content);
    }
}
