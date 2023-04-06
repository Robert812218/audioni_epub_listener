use std::io;
use std::io::Read;
use std::fs;


fn main() {
   println!("Welcome to the audio app");
   print!("\n");
   println!("Select your book: ");
   print!("\n");

   // display contents of library, user selects one 
   let paths = fs::read_dir("src/library/").unwrap();
    
   let mut counter = 1;
    
   for path in paths {
    println!("Name: {}, id: {}", path.unwrap().path().display(), counter);
    counter += 1;
   }

   let mut select = String::new();

   io::stdin().read_line(&mut select).expect("Unrecognized selection");

   println!("Book selected: {}", select);

   let mut file = std::fs::File::open("src/library/text1.txt").unwrap();
   let mut contents = String::new();
   file.read_to_string(&mut contents).unwrap();
   print!("{}", contents);
}
