# evcxr
[GitHub - google/evcxr](https://github.com/google/evcxr)
```
cargo install evcxr_repl
```

```
evcxr
```

## hello world
```
>> println!("Hello, world!");
```

## file io
```
>> use std::fs::File;
>> use std::io::prelude::*;
>> let path = "foo.txt";
>> let mut file = File::create(path)?;
>> let contents = "Hello, world!";
>> file.write_all(contents.as_bytes())?;
>> println!("written = {}", contents);
>> let mut file = File::open(path)?;
>> let mut contents = String::new();
>> file.read_to_string(&mut contents)?;
>> println!("read = {}", contents);
>> fs::remove_file(path)
```

## rand
```
>> :dep rand = { version = "0.7.3" }
>> use rand::prelude::*;
>> let mut rng = rand::thread_rng();
>> let y: f64 = rng.gen();
>> println!("y = {}", y);

>> let mut nums: Vec<i32> = (1..100).collect();
>> nums.shuffle(&mut rng);
>> println!("nums = {:?}", nums);
```

## serde
```
>> :dep serde = { version = "1.0", features = ["derive"] }
>> :dep serde_json = { version = "1.0" }
>> use serde::{Serialize, Deserialize};
>> #[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}
>> let point = Point { x: 1, y: 2 };
>> let serialized = serde_json::to_string(&point).unwrap();
>> println!("serialized = {}", serialized);
>> let deserialized: Point = serde_json::from_str(&serialized).unwrap();
>> println!("deserialized = {:?}", deserialized);
```

