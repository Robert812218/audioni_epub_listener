use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};

use epub::doc::EpubDoc;
// use postgres::{Client, NoTls};
use std::env;

mod library;
mod doc_interact;

use crate::library::TextFile;
use crate::library as my_library;

const APP_ID: &str = "org.gtk_rs.Hello_World2";

fn main() -> glib::ExitCode {
	let app = Application::builder().application_id(APP_ID).build();

	app.connect_activate(build_ui);

	app.run()
}

fn build_ui(app: &Application) {
	let window = ApplicationWindow::builder()
		.application(app)
		.title("My GTK App")
		.build();

	window.present();
}

// fn main() -> Result<(), postgres::Error> {
//     let password = env::var("audioni_DB_PASSWORD")
//         .expect("auioni_DB_PASSWORD environment variable not set");
//     let conn_string = "postgresql://audioni_user:audioni777@localhost/audioni";
//     let client = Client::connect(&conn_string, NoTls)?;

//    let text_files = library::create_library();
//    let selected_file_id = library::display_library(&text_files);

//    doc_interact::doc_interact(selected_file_id, &text_files);
    
    
//    Ok(())
// }
