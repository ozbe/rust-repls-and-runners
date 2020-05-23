# Rust REPLs and Runners

A collection of examples implemented with different Rust REPLs & Runners.

## Tools
- TODO

## Examples
### Hello World
#### Code
```
fn main() {
  println!("Hello, world!");
}
```

#### Dependencies
None

### File Example
Inspiration: [std::fs::File - Rust](https://doc.rust-lang.org/std/fs/struct.File.html)

#### Code
```
use std::fs::{self, File};
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
  let path = "foo.txt";

  let mut file = File::create(path)?;
  let contents = "Hello, world!";
  file.write_all(contents.as_bytes())?;
  println!("written = {}", contents);

  let mut file = File::open(path)?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;
  println!("read = {}", contents);

  fs::remove_file(path)
}
```

#### Dependencies
None

### Rand Example
Inspiration: [rand - Rust](https://docs.rs/rand/0.7.3/rand/)

```
use rand::prelude::*;

fn main() {
  let mut rng = rand::thread_rng();
  let y: f64 = rng.gen();
  println!("y = {}", y);

  let mut nums: Vec<i32> = (1..100).collect();
  nums.shuffle(&mut rng);
  println!("nums = {:?}", nums);
}
```

#### Dependencies
```
rand = "0.7.3"
```

### Serde Example
Inspiration: [Overview · Serde](https://serde.rs/)

```
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 1, y: 2 };
    let serialized = serde_json::to_string(&point).unwrap();
    println!("serialized = {}", serialized);
    let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
}
```

#### Dependencies
```
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## Contributing

TODO

## License

TODO
