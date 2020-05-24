# runner

[Examples](../README.md#examples) implemented with [runner](https://github.com/stevedonovan/runner).

## Prereqs

- [runner](https://github.com/stevedonovan/runner)

## Examples
### Hello World

#### Source

[hello-world.rs](hello-world.rs)

#### Run
```
runner hello-world.rs
```

### File Example

#### Source

[file-example.rs](file-example.rs)

#### Run
```
runner file-example.rs
```

### Rand Example

#### Source

[rand-example.rs](rand-example.rs)

#### Setup
```
$ echo 'rand = "0.7.3"' >> ~/.cargo/.runner/static-cache/Cargo.toml
$ runner --build
```

#### Run
```
runner rand-example.rs
```

### Serde Example

#### Source

[serde-example.rs](serde-example.rs)

#### Setup
```
$ echo 'serde = { version = "1.0", features = ["derive"] }' >> ~/.cargo/.runner/static-cache/Cargo.toml
$ echo 'serde_json = "1.0"' >> ~/.cargo/.runner/static-cache/Cargo.toml
$ runner --build
```

#### Run
```
runner -s --macro serde -x serde_json serde-example.rs
```