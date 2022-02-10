# borrow-fix

Is the borrow checker getting in your way?

This crate fixes that.


## Example:

without borrowfix:

```rust
fn printit(s :String) {
	println!("{}",s);
}

let s = String::new();
printit(s);
println!("s is {}",s); // Wont compile :(
```

with borrowfix:

```rust
use borrowfix::remember;

fn printit(s :String) {
	println!("{}",s);
}

let s = String::new();
let refer = remember(&s);
printit(s);
println!("refer is {}",refer); // Works fine
```

## Safety

This crate is compleatly sound. It wont causÊÃúý$HOME/projects/borrowf[SEGMENTATION FAULT]
