# runner

[Examples](https://github.com/ozbe/rust-repls-and-runners#examples) implemented with [runner](https://github.com/stevedonovan/runner).

## Prereqs

- [runner](https://github.com/stevedonovan/runner)

## Examples
### Hello World

#### Run
```
runner hello-world.rs
```

### File Example

#### Run
```
runner file-example.rs
```

### Rand Example

#### Setup
```
$ echo 'rand = "0.7.3"' >> ~/.cargo/.runner/static-cache/Cargo.toml
$ runner --build
```

#### Run
```
runner rand-example.rs
```
https://github.com/stevedonovan/runner/issues/4

## #Serde Example

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