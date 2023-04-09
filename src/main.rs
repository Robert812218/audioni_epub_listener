use postgres::{Client, NoTls};
use std::env;

mod library;

fn main() -> Result<(), postgres::Error> {
    let password = env::var("audioni_DB_PASSWORD")
        .expect("auioni_DB_PASSWORD environment variable not set");
    let conn_string = "postgresql://audioni_user:audioni777@localhost/audioni";
    let client = Client::connect(&conn_string, NoTls)?;

    let text_files = library::create_library();
    library::display_library(&text_files);

    Ok(())
}

   //  let text_files = library::create_library();
   //
    // library::display_library(&text_files);
