use std::io;
use std::io::Read;
use std::fs;

#[derive(Debug)]
struct MyFiles {
    file_names: Vec<String>,
}

fn main() -> std::io::Result<()> {
    let mut my_files = MyFiles {
        file_names: Vec::new(),
    };

    let paths = fs::read_dir("src/library/")?;
    
    for path in paths {
        let file_name = path?.file_name().into_string().unwrap();
        my_files.file_names.push(file_name);
    }
    
    my_files.file_names.sort();

    println!("{:?}", my_files);

    Ok(())
}
