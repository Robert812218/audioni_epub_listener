use std::fs;
use std::io::{BufReader, Read};
use std::path::Path;

struct TextFile {
    title: String,
    content: String,
    id: u8,
}

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

    println!("Your library:");
    for text_file in &text_files {
        println!("Name: {}, id: {}", text_file.title, text_file.id);
    }

    let mut input = String::new();
    println!("Enter the id of the file you want to read:");
    std::io::stdin().read_line(&mut input).unwrap();

    let selected_id = input.trim().parse::<u8>().unwrap();

    let selected_file = text_files.iter().find(|f| f.id == selected_id);

    match selected_file {
        Some(file) => println!("{}", file.content),
        None => println!("File not found."),
    }
}

