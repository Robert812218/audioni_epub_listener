use crate::library::TextFile;

pub fn doc_interact(selected_file_id: u8, text_files: &[TextFile]) {
    let selected_file = text_files.iter().find(|file| file.id == selected_file_id);
    
    if let Some(file) = selected_file {
        println!("{}", file.content);
    } else {
        println!("File not found.");
    }
}
