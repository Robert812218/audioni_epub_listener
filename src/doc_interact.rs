use std::io;

mod library;

pub fn doc_interact(selected_file_id: u8, text_files: &[library::TextFile]) {
	let selected_file = text_files.iter().find(|file| file.id == selected_file.id);

	if let Some(file) = text_files.iter().find(|file| file.id == selected_file_id) {
	
	}

	if let Some(file) = selected.file {
		println!("Selected file: {}", file.title);
		println!("{}", file.content);
	} else {
		println!("Invalid file ID: {}", selected_file_id);
		return;
	}

	let mut input = String::new();
	println!("Enter a command: ");
	io::stdin().read_line(&mut input).expect("Failed to read input");
}
