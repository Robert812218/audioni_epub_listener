mod library;

fn main() {
    let text_files = library::create_library();
    library::display_library(&text_files);
}
