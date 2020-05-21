# Rust REPL and Runners

## Code examples
### hello world
```
fn main() {
  println!("Hello, world!");
}
```

### file io
Source: [std::fs::File - Rust](https://doc.rust-lang.org/std/fs/struct.File.html)
```
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
  write()?;
  read()   
}

fn write() {
  let mut file = File::create("foo.txt")?;
  file.write_all(b"Hello, world!")?;
  Ok(())
}

fn read() {
  let mut file = File::open("foo.txt")?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;
  assert_eq!(contents, "Hello, world!");
  Ok(())
}
```

### rand
Source: [rand - Rust](https://docs.rs/rand/0.7.3/rand/)
```
use rand::prelude::*;

if rand::random() { // generates a boolean
    // Try printing a random unicode code point (probably a bad idea)!
    println!("char: {}", rand::random::<char>());
}

let mut rng = rand::thread_rng();
let y: f64 = rng.gen(); // generates a float between 0 and 1

let mut nums: Vec<i32> = (1..100).collect();
nums.shuffle(&mut rng);
```

### serde
Source: [Overview · Serde](https://serde.rs/)
```
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 1, y: 2 };

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&point).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Point.
    let deserialized: Point = serde_json::from_str(&serialized).unwrap();

    // Prints deserialized = Point { x: 1, y: 2 }
    println!("deserialized = {:?}", deserialized);
}
```

