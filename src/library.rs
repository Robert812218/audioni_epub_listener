use std::fs;
use std::io::{BufReader, Read};
use std::path::Path;
use std::io;

pub struct TextFile {
    pub title: String,
    pub content: String,
    pub id: u8,
}

pub fn read_file(path: &Path) -> Result<String, std::io::Error> {
    let file = fs::File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content)?;
    Ok(content)
}

pub fn create_library() -> Vec<TextFile> {
    let folder_path = Path::new("src/library");
    let mut text_files = Vec::new();

    let mut id_count: u8 = 0;

    for entry in fs::read_dir(folder_path).unwrap() {
        let entry = entry.unwrap();
        let file_path = entry.path();
        let file_name = entry.file_name().into_string().unwrap();

        if file_path.is_file() {
            if let Ok(content) = read_file(&file_path) {
                id_count += 1;
                let text_file = TextFile {
                    title: file_name,
                    content: content,
                    id: id_count,
                };
                text_files.push(text_file);
            }
        }
    }

    text_files
}

pub fn display_library(text_files: &[TextFile]) {
    println!("Enter the id of the file you want to view:");

    for text_file in text_files {
        println!("id: {}, name: {}", text_file.id, text_file.title);
    }


    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let file_id: u8 = match input.trim().parse() {
        Ok(id) => id,
        Err(_) => {
            println!("Invalid input, please enter a valid file id.");
            return;
        }
    };

    match text_files.iter().find(|f| f.id == file_id) {
        Some(text_file) => println!("{}", text_file.content),
        None => println!("File with id {} not found.", file_id),
    }
}
