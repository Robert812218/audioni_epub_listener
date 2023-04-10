use epub::doc::EpubDoc;
// use postgres::{Client, NoTls};
use std::env;

mod library;
mod doc_interact;

fn main() -> Result<(), postgres::Error> {
//     let password = env::var("audioni_DB_PASSWORD")
//         .expect("auioni_DB_PASSWORD environment variable not set");
//     let conn_string = "postgresql://audioni_user:audioni777@localhost/audioni";
//     let client = Client::connect(&conn_string, NoTls)?;

        

    let text_files = library::create_library();
    let selected_file_id = library::display_library(&text_files);

    doc_interact::doc_interact(selected_file_id, &text_files);
    
    
    Ok(())
}
