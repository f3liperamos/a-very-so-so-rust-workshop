# Clone this repo
> git clone https://github.com/f3liperamos/dotfiles.git

# Docker
> docker build -t sb-rust . && docker run -v $PWD/code:/code -it --rm sb-rust
PS: `cargo run` command is painfully slow when running docker with volumes on MacOS, it's taking > 20 seconds to build.

# Install Rust locally
> curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

[Recommended] install cargo-watch so you don't have to execute `cargo run` after every file change
> cargo install cargo-watch

# Let's code
Rust has a steep learning curve and it takes time to get used to the compiler screaming at you for every little mistake.

There are a few dummy exercises dummy exercises that I definitely didn't steal from [Tour of Rust](https://tourofrust.com/) or [Rustlings](https://github.com/rust-lang/rustlings).

The ultimate goal is to solve the challenge from the [Advent of Code 2020 - day 1](https://adventofcode.com/2020/day/1). You can use your own input from Advent of Code or use the predefined one I left there.

All exercise will have a brief explanation above them
....


# Better "let's code" sources
- https://github.com/rust-lang/rustlings
- https://tourofrust.com/
- https://rust-unofficial.github.io/too-many-lists/
- https://www.codewars.com/?language=rust


# EXTRA: More of basic syntax (really basic)

### "console.log"
```rust
println!("Hello, I'm Rust's \"console.log\"");

// printing variables
let variable = "handsome";
println!("Hello, {}", variable); // ‣Hello handsome
```

### Variables
```rust
const VARIABLE = String::from("HELLO WORLD!"); // constants, unlike javascript once defined they don't change by any means
let my_variable = String::from("HELLO WORLD!"); // variable, however, unlike javascript it doesn't mutate unless you tell it can mutate

let mut my_mut_variable = String::from("HELLO WORLD!");// see the MUT keyword there? Now we can mutate
my_mut_variable = String::from("Hell yeah!");
```

### Arrays - in Rust, arrays always have a known size at compile-time and must contain a single type only
```rust
let array_1: [i32; 2] = [10, 20];
let array_2 = [1.1, 1.2, 1.3]; // ‣array_2: [f64; 3] inferred
let array_3 = ["A", "B", "C"]; // ‣array_3: [&str; 3] inferred
```

### Tuples
```rust
let my_tupple = (10, "string");
println!("{}, {}", my_tupple.0, my_tupple.1);
```

### Loops 
while
```rust
let mut i = 0;

while i < 10 {
  println!("i = {}", i);
  i = i + 1;
}
```

loop - just like while but loops indefinitely until you break
```rust
let mut i = 0;
loop {
  if i > 10 {
      break;
  }
  i += 1;
}
```

for
```rust
let array = [1, 2, 3, 4, 5];
for number in array {
  println!("number is {}", number);
}
```

### Control flow
if/else
```rust
let x = 0;
if x > 1 { println!("x>1") } else { println!("x<1") }
```

pattern matching
```rust
let x = 1;
match x {
  1 => println!("one"),
  2 => println!("two"),
  3 => println!("three"),
  4 => println!("four"),
  5 => println!("five"),
  _ => println!("something else"),
}
```
