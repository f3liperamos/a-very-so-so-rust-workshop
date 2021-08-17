# [Clone this repo](#clone)
> git clone https://github.com/f3liperamos/a-very-so-so-rust-workshop.git

# [Use docker](#docker)
> docker build -t sb-rust . && docker run -v $PWD/code:/code -it --rm sb-rust

PS: `cargo run` command is painfully slow when running docker with volumes on MacOS, it's taking > 20 seconds to build.

# [OR Install Rust locally](#install-glorious-rust)
> curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

[Recommended] install cargo-watch so you don't have to execute `cargo run` after every file change
> cargo install cargo-watch

# [Let's code](#lets-code)
Rust has a steep learning curve, it takes time to get used to the compiler screaming at you for every little mistake.
 Try to remember which methods would you use if you were programming in javascript [Rust docs](https://doc.rust-lang.org/std/index.html) is your best friend,
 just type there "reduce", "map" or anything and find the correct struct you need. If you're looking for mapping array [array::map](https://doc.rust-lang.org/std/primitive.array.html#method.map)
 might be the one you're looking for.

I put there two exercises that I totally didn't steal from [CodeWars](https://www.codewars.com/?language=rust), and one puzzle from
 the [Advent of Code 2020](https://adventofcode.com/2020). Let's see if we are able to solve at least one of them. All of them have an explanation on top of the exercise

# [Learning material](#learning)
- [Rust official documentation - https://doc.rust-lang.org/std/index.html](https://doc.rust-lang.org/std/index.html), leading the list with our beloved [RTFM approach](https://en.wikipedia.org/wiki/RTFM)
- [A Gentle Introduction To Rust - https://stevedonovan.github.io/rust-gentle-intro/readme.html](https://stevedonovan.github.io/rust-gentle-intro/readme.html)
- [Tour of Rust - https://tourofrust.com](https://tourofrust.com), It has a nicer pacing compared with the official book and gives you a sandbox
- [Rust official book - https://doc.rust-lang.org/book/title-page.html](https://doc.rust-lang.org/book/title-page.html), great but MASSIVE (and that's the reason why I didn't put at the top of the list)
- [Rust Cheat sheet - https://cheats.rs](https://cheats.rs/), what a WONDERFUL website
- [Rust cookbook - https://rust-lang-nursery.github.io/rust-cookbook](https://rust-lang-nursery.github.io/rust-cookbook/), a collection of simple examples of common programming tasks
- [Rust by example - https://doc.rust-lang.org/rust-by-example](https://doc.rust-lang.org/rust-by-example/), One of the official books
- [ctjhoa/rust-learning - https://github.com/ctjhoa/rust-learning](https://github.com/ctjhoa/rust-learning), A bunch of links to blog posts, articles, videos, etc for learning Rust. Including the above

# [Some better "let's code" sources](#better-lets-code)
- https://github.com/rust-lang/rustlings
- https://rust-unofficial.github.io/too-many-lists/
- https://www.codewars.com/?language=rust

# [EXTRA: More of basic syntax (really basic)](#really-basic-syntax)
I REALLY recommended you to use any of the sources I put in [Learning Material](#learning) or the official source/book instead. As a quick-check source, it may work

### "console.log"
```rust
println!("Hello, I'm Rust's \"console.log\"");

// printing variables
let variable = "handsome";
println!("Hello, {}", variable); // ‣Hello handsome

// you can also use named arguments
let bob = "Bob";
let greet = "how are you?";
println!("Hello {y}, {x}", x = greet, y = bob); // ‣ Hello Bob, how are you?
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

// To print arrays you need to use {:?}
println!("{:?}", array_1)

// not needed when accessing a specific index
println!("array_1[{}], array_2[{}]", array_1[0], array_2[1]) // ‣array_1[2], array_2[1.2]

// if you TRY to access a undefined index the code will not compile
println!("undefined index {}", array_1[2]) // compilation error index out of bounds
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
