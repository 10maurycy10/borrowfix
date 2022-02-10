note: This crate is a joke and under no circumstance should you actually use it (unless you want your stuff to break)

# borrow-fix

Is the borrow checker getting in your way?

This crate fixes that.


## Example:

without borrow-fix:

```rust
fn printit(s :String) {
	println!("{}",s);
}

let s = String::new();
printit(s);
println!("s is {}",s); // Wont compile :(
```

with borrow-fix:

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

This crate is completely sound. It wont causÊÃúý$HOME/projects/borrowf[SEGMENTATION FAULT]
