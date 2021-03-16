# Ut

Wraps asynchronous functions in order to make them synchronous based on if a
"sync" feature is enabled. This is useful when writting http clients as you
can write async methods and wrap them for use in synchronous code bases
automatically.

# Usage
```toml
[dependencies]
ut = "0.2.0"
```

Then in the crate that you want to have synchronous functions created for you
you create a sync feature. When this feature is enabled ut will create
synchronous functions on what you have wrapped.

you can either: 
 - Replace your asynchronous function with a synchronous one
 - Clone your asynchronous function with a synchronous one ending in blocking
 - Clone all methods in an impl with synchronous ones

# Replacing async functions

You can replace your asynchronous function with a synchronous one by doing
something like the following:


```rust
#[ut::wrap]
async fn foo(input: &str) -> String {
  format!("I am {} now", input)
}

fn main() {
  let out = foo("sync");
  assert_eq!(out, "I am sync now".to_owned())
}
```

# Cloning async functions 

You can clone your asynchronous function with a synchronous one ending in blocking
by doing something like the following:

```rust
#[ut::clone]
async fn foo(input: &str) -> String {
 format!("I am {} now", input)
}

let out = foo_blocking("sync");
assert_eq!(out, "I am sync now".to_owned())
```

# Cloning async methods in implementations

You can clone all methods in an impl with synchronous ones by using
ut::clone_impl. This is useful when you want to support both
async and sync functions in a struct implementation.


```rust
// The original struct
#[derive(Default)]
pub struct Example {
  pub fooers: Fooers,
}

// You also need to create the struct to place the cloned impls in
// This is done so you can choose what structs/impls to clone/wrap
// The cloned structs/impls should end in Blocking
#[derive(Default)]
pub struct ExampleBlocking {
  pub fooers: FooersBlocking,
}

// The original struct with async functions
#[derive(Default)]
pub struct Fooers;

// The blocking struct that we are cloning impls into
// You have to create this so you can add custom derives
#[derive(Default)]
pub struct FooersBlocking;

// The async impls that you want to wrap
// All methods within this impl must be async
#[ut::clone_impl]
impl Fooers {
  pub async fn foo(&self, input: &str) -> String {
    format!("I am {} now", input)
  }

  pub async fn bar(&self, input: &str) -> String {
    format!("I am also {} now", input)
  }
}
let example = ExampleBlocking::default();
let out = example.fooers.foo("sync");
assert_eq!(out, "I am sync now".to_owned());
let out = example.fooers.bar("sync");
assert_eq!(out, "I am also sync now".to_owned())
```

Currently the wrapping is very naive and simply wraps the function in
tokio::main. This is likely more expensive then it needs to be and I hope
to make it more efficient later.

