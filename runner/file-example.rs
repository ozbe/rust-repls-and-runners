use std::io::prelude::*;

let path = "foo.txt";

let mut file = File::create(path)?;
let contents = "Hello, world!";
file.write_all(contents.as_bytes())?;
println!("written = {}", contents);

let mut file = File::open(path)?;
let mut contents = String::new();
file.read_to_string(&mut contents)?;
println!("read = {}", contents);

fs::remove_file(path)?;