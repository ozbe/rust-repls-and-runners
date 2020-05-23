# evcxr

[Examples](../README.md#examples) implemented with [evcxr](https://github.com/google/evcxr)

## Prereqs

- [evcxr](https://github.com/google/evcxr)

### Hello World

#### Run
```
$ evcxr
>> println!("Hello, world!");
```

### File Example

#### Run
```
$ evcxr
>> use std::fs::{self, File};
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
>> fs::remove_file(path);
```

### Rand Example

#### Run
```
$ evcxr
>> :dep rand = { version = "0.7.3" }
>> use rand::prelude::*;
>> let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
>> let y: f64 = rng.gen();
>> println!("y = {}", y);
>> let mut nums: Vec<i32> = (1..100).collect();
>> nums.shuffle(&mut rng);
>> println!("nums = {:?}", nums);
```

### Serde Example

#### Run
```
$ evcxr
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

