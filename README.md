# Syncwrap

Wraps asynchronous functions in order to make them synchronous based on if a
"sync" feature is enabled. This is useful when writting http clients as you
can write async methods and wrap them for use in synchronous code bases
automatically.

# Usage
```toml
[dependencies]
syncwrap = "0.2.0"
```

# Examples
```rust
#[syncwrap::wrap]
async fn foo(input: &str) -> String {
  format!("I am {} now", input)
}

fn main() {
  let out = foo("sync");
  assert_eq!(out, "I am sync now".to_owned())
}
```

Currently the wrapping is very naive and simply wraps the function in
tokio::main. This is likey more expensive then it needs to be and I hope
to make it more efficient later.



