Hi, this site is still in very early development. You can check its [source code](https://github.com/xoko14/shoudev) while you wait.

Also, check out this server-side rendered code snippet:

```rust
use rand::prelude::*;

fn main() {
    let mut rng = rand::thread_rng();

    let is_greet: bool = rng.gen();
    
    if is_greet {
        println!("hello world!");
    }
    else {
        println!("goodby world!")
    }
}
```
