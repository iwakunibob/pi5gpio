# Rust Language Summary

## Installation

```bash
curl https://sh.rustup.rs -sSf | sh
```

## Create new project

```bash
cargo new my_project
```

This creates a folder called my_project with files:

* Cargo.toml=Project settings
* src/main.rs: Main Rust file
* The top-level project directory for README, license, and configuration files

### Build and Run Project

rustc main.rs  #compiles simples program with no dependencies.
cargo init  # Creates a Cargo.toml project file manually
cargo --version

Filename: Cargo.toml 
The last line, `[dependencies]`, is the start of a section for you to list any
of your project’s dependencies. In Rust, packages referred to as *crates*.

cargo check   # checks for errors without building executable

cargo build    # compiles and makes executable

Project ready for release, use `cargo build --release` to compile with optimizations.

follow by cargo run --release

`cargo run` # Builds and runs program with debug included

## Rust Syntax

### Naming Conventions

- Rust prefers **snake_case** for *variables*, *functions* and *crates.*

- *Types, Traits, and Enum*: use **UpperCamelCase** (PascalCase)

- Macros: Use **snake_case!** end with !

- Constants & Statics: Use **SCREAMING_SNAKE_CASE**

- Functions & Methods: Use **snake_case**

  - Conversions should be provided as methods:

  | Prefix  | Cost      | Ownership                          |
  | ------- | --------- | ---------------------------------- |
  | `as_`   | Cheap     | Borrowed -> borrowed.              |
  | `to_`   | Expensive | Borrowed -> owned); owned -> owned |
  | `into_` | Variable  | Owned -> owned (non-Copy types).   |

  - General constructors: Use `new`

  - Conversion constructors: Use `from_some_other_type`

  -  `get_` prefix is not used for getters

  - Iterators should be labeled as:

    - fn iter(&self) -> Iter        

    - fn iter_mut(&mut self) -> IterMut  

    - fn into_iter(self) -> IntoIter  

### Variables and Mutability

Variables are immutable, by default. Make mutable with `mut`.

Constants are never mutable can show a calculation and usually UPPER_CASE. Valid entire time program runs, within scope as declared. Must provide data type

```rust
fn main() {  // Function main
    println!("Hello, Bob!"); // println!  is a macro for output
    print!("No new line after. ");
    print!("No new line \nafter\n"); 
    let name: &str = "Totoro";
    let mut age: u32 = 69;
    age += 1;
    println!("{} is {} years old.", name, age);
/*  Variables are immutable by default. 
Must precede with mut to make a variable mutable
Constants must have Type specified
*/
    const BIRTHYEAR: i32 = 1980; 
}
```

Shadow variables have duplicate names within same scope and both declared let. 
Literals for describing different bases.

| Literals | Decimal | Hex  | Octal | Binary      | Byte (`u8` only) |
| -------- | ------- | ----:| ----- | ----------- | ---------------- |
| Examples | 98_222  | 0xff | 0o77  | 0b1111_0000 | b'A'             |

### Scaler Data Types

| Length                 | Signed Int | Unsigned int | Floating | Others |
| ---------------------- | ---------- | ------------ | -------- | ------ |
| 8-bit                  | `i8`       | `u8`         |          | `bool` |
| 16-bit                 | `i16`      | `u16`        |          |        |
| 32-bit                 | **i32**    | **u32**      | `f32`    | `char` |
| 64-bit                 | `i64`      | `u64`        | **f64**  |        |
| 128-bit                | `i128`     | `u128`       |          |        |
| Architecture-dependent | `isize`    | `usize`      |          |        |

Bool type consumes 1 byte. Char type is 4 bytes.

### Compound Types

*Compound types* can group multiple values into one type, as is with primitive types Tuples and Arrays.

#### Tuples

A tuple holds a multiple values of different types and fixed length. It is useful when grouping different types together. You access tuple elements using a dot `.` and index number. Tuple packed when declared. Can unpack it into seperate variables. 

The tuple without any values has a special name, *unit*. This value and its corresponding type are both written `()` and represent an empty value or an empty return type.

```rust
fn main() {
  let person: (&string, u8, bool) = ("John", 30, true); // Pack tuple
  println!("Name: {}", person.0);  // values accessed by . notation
  println!("Age: {}", person.1);
  println!("Is active: {}", person.2);
  let (name, age, active) = person; // Unpack a tuble into variables
  println!("Name: {}", name);
  println!("Age: {}", age);
  println!("Active: {}", active);
}
```

Tuples are often used to return multiple values from a function:

```rust
fn get_user() -> (String, String, u8) {
  (String::from("Bob"), "Laurie".to_string(), 25)
}

fn main() {
  let user = get_user();
  println!("User: {} {} is {} years old", user.0, &user.1, user.2);
}
```

#### Arrays

An array in Rust is a fixed-size list of values, all of the same type.
Array indexes [0] is the first element, [1] is the second element, etc.

Arrays are useful when you want your data allocated on the stack, the same as the other types we have seen so far, rather than the heap.

```rust
fn main() {
  let mut fruits = ["apple", "banana", "orange"];
  let a: [i32; 5] = [1, 2, 3, 4, 5];
  let b = [0; 5]; // Array b intialized to [0, 0, 0, 0, 0]
  fruits[1] = "pear";
  print!("{:?} is size {}\nI like ", fruits, fruits.len());
  for fruit in fruits {
    print!("{}, ", fruit);
  }
}
```

### Key Precedence Rules:

- **Highest Precedence:** **Paths**, **Method calls**, and **Field expressions** 

- **Unary Operators:** Unary `-`, `*`, `!`, `&`, `&mut` have high precedence.

- **Type Casting:** The `as` operator is left-associative.

- **Multiplicative:** `*`, `/`, `%` (Left-to-Right).

- **Additive:** `+`, `-` (Left-to-Right).

- **Bitwise Shifts:** `<<`, `>>` (Left-to-Right).

- **Bitwise AND:** `&` (Left-to-Right).

- **Bitwise XOR:** `^` (Left-to-Right).

- **Bitwise OR:** `|` (Left-to-Right).

- **Comparison:** `==`, `!=`, `<`, `<=`, `>`, `>=` require parentheses if chained

- **Logical AND:** `&&` (Left-to-Right).

- **Logical OR:** `||` (Left-to-Right).

- **Ranges:** `..`, `..=` require parentheses in complex expressions.

- **Assignment:** `=`, `+=`, `-=`, etc. are **Right-to-Left**.

- **Lowest Precedence:** `return`, `break`, and closures.

### Conditional Control Structures

```rust
let score = 85;
if score >= 90 {
  println!("Grade: A");
} else if score >= 80 {
  println!("Grade: B");
} else if score >= 70 {
  println!("Grade: C");
} else {
  println!("Grade: F");
}
```

In Rust, `if...else` can also be used as an **expression**. You can assign the result of an `if` to a variable. Rust does not have a ternary operator, but using `if...else` as an expression gives you the same effect.

```rust
let time = 20;
let greeting = if time < 18 { "Good day." } else { "Good evening." };
println!("{}", greeting);
```

#### Match similar to select-case in other languages

- The `match` variable (**day**) is evaluated once.
- The value of the day variable is compared with the values of each "branch"
- Each branch starts with a value, followed by `=>` and a result
- If there is a match, the associated block of code is executed
- `_` is used to specify some code to run if there is no match (like `default` in other languages)
- You can match multiple values at once using the `|` operator (OR):
- Just like `if`, `match` can also return a value:

```rust
fn main() {
  let day = 6;
  match day {
    1 | 2 | 3 | 4 | 5 => println!("Weekday"),
    6 | 7 => println!("Weekend"),
    _ => println!("Invalid day"),
  };
}
```

### Repetition Control Structures

#### loop

The simplest of Rust's three loop types. It will run forever unless you `break;`
You can also return a value from a `loop` using `break` with a value.
This lets you save the result of the loop into a variable:

```rust
fn main() {
  let mut count = 1;
  let result = loop {
    println!("Hello!");
    if count == 3 {
      break count; // Stop the loop and return the number 3
    }
    count += 1;
  };
  println!("The loop stopped at: {}", result);
}
```

#### while

The `while` loop runs as long as a condition is `true`. You can stop a `while` loop when you want by using `break;` or skip a value by using the `continue;` statement.

```rust
fn main() {
  let mut num = 1;
  while num <= 10 {
    if num == 6 {
      num += 1;
      continue;
    }
    println!("Number: {}", num);
    num += 1;
  }
}
```

#### for

When you know exactly how many times you want to loop through a block of code, use the `for` loop together with the `in` keyword. 

Rust handles the counter variable (`i`) automatically, unlike many other programming languages. You don't need to declare or increment it manually. 

`1..6` means from 1 up to (but not including) 6. If you want to include the last number, use `..=` (two dots and an equals sign) 

Just like other loops, you can use `break` to stop the loop and `continue` to skip 

```rust
fn main() {
  for i in 1..=10 {
    if i == 3 {
      continue; // skip 3
    }
    if i == 5 {
      break; // stop before printing 5
    }  
    println!("i is: {}", i);
  }
}
```

### Functions

To create a function, use the `fn` keyword, function name and a set of parentheses `()` and curly braces `{}`

To call a function, write the name of the function followed by two parantheses `()`.

Parameters are written inside the parentheses `(name: &str, age: u32)` 

Use the  `->` symbol in the function header to show type of value returned.

In Rust, you may omit the `return` keyword. Just write the value on the last line of the function, **without a semicolon**. Either way works.

```rust
fn main() {
  let sum = add(3, 4);
  println!("Sum is: {}", sum);
}
fn add(a: i32, b: i32) -> i32 {
  return a + b;
}
```

#### Scope

A variable only lives inside the block where it was created. A block is anything inside curly braces `{ }`. Variables created inside a function only exists inside the function.

You can also create blocks inside other code, like in if, for, or while statements. Variables created in these blocks are only valid inside them.

In Rust, you **can** declare a new variable with the same name in the same scope using `let`. This is called **shadowing**. The second `x` replaces the first one. The value `5` is no longer accessible after the second declaration.  In Rust, it's a feature used to transform or update values safely.

You can also reuse a variable name inside a new block:

```rust
fn main() {
  let x = 5;
  {
    let x = 10;
    println!("Inside block: {}", x);
  }
  println!("Outside block: {}", x);
}
```



## Ownership

Some languages have garbage collection that regularly looks for no-longer-used
memory as the program runs; in other languages, the programmer must explicitly
allocate and free the memory. Rust uses a third approach: Memory is managed
through a system of ownership with a set of rules that the compiler checks. 

All data stored on the *stack* must have a known, fixed size. Data with an unknown
size at compile time or a size that might change must be stored on the *heap*. Stack data access is faster than heap. 

The error **E0382: borrow of moved value** occurs because you passed a variable by value to the `leds_left` function, transferring ownership and invalidating the original variable. The compiler suggests **borrowing** because the function likely only needs to read the data, not own it.

To fix this, change the function signature to accept a reference instead of taking ownership.

```
// Before (Incorrect)
fn leds_left(leds: Vec<u8>) { ... }

// After (Correct)
fn leds_left(leds: &Vec<u8>) { ... }
// Or more idiomatically:
fn leds_left(leds: &[u8]) { ... }
```

When calling the function, pass the variable with an ampersand:

```
let my_leds = vec![1, 2, 3];
leds_left(&my_leds); // Passes a reference
println!("{}", my_leds.len()); // my_leds is still valid
```

If the function genuinely needs to own the data (e.g., to store it or modify it permanently), you must **clone** the value before passing it:

```
leds_left(my_leds.clone());
```

### Ownership rules

- Each value in Rust has an *owner*.

- You can only have one owner at a time, unless you [borrow](https://www.w3schools.com/rust/rust_borrowing_references.php) it .

- When the owner goes out of scope, the value will be dropped.

- Simple types like numbers, characters and booleans are **copied**, not moved.
  This means you can still use the original variable after assigning it to another.

- Complex types like `String` are moved to new variable, old variable deleted.

### Strings

There are two main types of strings in Rust:

- `&str` - is called "string slices", and is literal fixed text like `"Hello"`
- **String** - type used when you need a string that can change

You can create a `String` from a string literal using the `.to_string()` method 
or the `String::from()` function. Operator :: is a namespace from String type.

Strings are mutable, so you can change them if they are declared with `mut`.

Use `push_str()` to add text to a string. Use `push()` to add one character.

```rust
fn main() {
  let mut greeting = String::from("Hello"); // Allocates heap memory
  greeting.push_str(" World");
  greeting.push('!');
  println!("{}", greeting);
}
```

#### Concatenating Strings

You can combine strings using the `format!` macro or the `+` operator.  You can only add a `&str` to a `String` with `+`. That is why `&s2` and `&s3` (a string slice) is used.

You can use the `.len()` method to get the length of a string

```rust
fn main() {
  let s1 = String::from("Hello");
  let s2 = "World!".to_string();
  let s3 = String::from("What a beautiful day!");
  let result1 = format!("{} {} {}", s1, s2, s3);
  let result2 = s1 + ", " + &s2 + ", " + &s3;
  println!("{} is length {}", result1, result1.len());
  println!("{} is length {}", result2, result2.len());
}
```

#### Clone

For non-simple  types, like `String`, allows keeping the original variable and value and also assign value to another variable, you can use the `.clone()` method, which makes a copy of the data and does not delete old variable. If you don't need to own the value twice, using a reference (`&`) is usually better than cloning,

##### Borrowing using a Reference

A reference lets you look at a value without owning it. You create a reference using the `&` symbol. Since `b` is only borrowing the value, `a` still owns it.

To **change** a value through a reference, you need to make the reference `mut`. 
You can only have one mutable reference to a value at a time!

```rust
fn main() {
  let mut name = String::from("John");
  let name_ref = &mut name;
  name_ref.push_str(" Doe");

  println!("{}", name_ref); // John Doe
  println!("{}", name); // John Doe
}
```

Borrowing helps you reuse values safely, without giving them away.

- It lets you use values without taking ownership
- It avoids cloning, which can be slow for large data
- It makes your programs safer and faster

## Data Structures

In Rust, data structures are used to store and organize values.

For println! use `{}` for basic types like strings, numbers, and booleans, and 
`{:?}` for whole data structures like arrays and vectors

### Vectors

A vector is a resizable array. Vectors can grow or shrink in size and are mutable.
To create a vector, use the `vec!` macro. 
Methods .push()  .pop()  .insert()  .remove()  .len()  can be used with vectors.

When you loop through a vector without `&`, values are moved out, and vector deleted. But when you borrow vector using `&`, it is still available in program.

```rust
fn main() {
  let mut fruits = vec!["apple", "banana", "orange"];
  fruits[1] = "pear";
  fruits.push("cherry"); // add single elements to end
  println!("{:?} is size {}", fruits, fruits.len());
  fruits.pop();  // remove single elements from end
  println!("{:?} is size {}", fruits, fruits.len());
  fruits.insert(1, "cherry"); // insert element at [1]
    println!("{:?} is size {}", fruits, fruits.len());
  fruits.remove(0); // remove element at [0]
  print!("{:?} is size {}\nI like ", fruits, fruits.len());
  for fruit in &fruits {
    print!("{}, ", fruit);
  }
  println!("\n{:?} is size {}", fruits, fruits.len());
}
```

### HashMaps

A HashMap stores key-value pairs. It lets you look up a value using a key. 
To use HashMap, you must import it from the standard library.

Methods  .insert()  .remove()  .get()  .len()  can be used with HashMaps.

```rust
use std::collections::HashMap;  // Import HashMap from std library
fn main() {
  let mut capitalCities = HashMap::new(); // Create empty HashMap
  capitalCities.insert("England", "London");  // Insert key-value
  capitalCities.insert("Germany", "Berlin");
  capitalCities.insert("Norway", "Oslo");
  capitalCities.insert("Japan", "Tokyo");
  capitalCities.insert("Canada", "Ottawa");
  capitalCities.insert("Canada", "Trump");  // Update values
  println!("{:?} of size {}", capitalCities, capitalCities.len());
  capitalCities.remove("Canada");			 // Remove key-value
  println!("{:?} of size {}", capitalCities, capitalCities.len());
  let mut country = "Norge";
  if let Some(city) = capitalCities.get(country) {
    println!("The capital of {} is {}.", country, city);
  } else {
    println!("{} is not in the map.", country);
  }
  for (country, city) in &capitalCities {
    println!("The capital of {} is {}.", country, city);
  }
}
```

#### To get a value from a `HashMap` in Rust

1. Safe Retrieval with Pattern Matching . This approach explicitly handles both the presence (`Some`) and absence (`None`) of the key

```rust
use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    // .get() takes a reference to the key and returns Option<&V>
    match scores.get("Blue") {
        Some(score) => println!("The score is: {score}"),
        None => println!("Team not found."),
    }
}
```

2. Use `.copied()` to turn the `Option<&V>` into an Option<V>, and then use `.unwrap_or()` to fall back to a default value if the key is missing.

```rust
use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    // Returns the integer 10, or 0 if "Red" doesn't exist
    let blue_score = scores.get("Blue").copied().unwrap_or(0);
    let red_score = scores.get("Red").copied().unwrap_or(0); 
    println!("Blue: {blue_score}, Red: {red_score}"); // Blue: 10, Red: 0
}
```

3. Direct Indexing (Panics if missing)

You can use the index operator (`[]`), which returns a direct reference to the value. **Warning:** If the key does not exist, your program will crash (panic). Only use this if you are absolutely certain the key exists.

```rust
use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    // Direct access via reference
    let score = &scores["Blue"];
    println!("Score: {score}");
 
}
```

### Structs

Custom data structure that groups related values together. Create a struct, then create object of it. Then, you can access the fields of the struct using dot syntax (`.`)

```rust
fn main() {
  struct Person {
    name: String,
    age: u32,
  }
  let mut user = Person {
    name: String::from("John"),
    age: 35,
  };
  user.age = 36;  // Change value of age
  println!("Name: {}", user.name);
  println!("Updated age: {}", user.age);
}
```

### Enums

Enumerations  is a way to define a type that can be one of a few different values. Each value in the enum is called a **variant**. Enums are useful when you want to represent a value that can only be one of a set of options - like days of the week, directions, or results like success and error.

```rust
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

fn main() {
  let my_direction = Direction::Up;

  match my_direction {
    Direction::Up => println!("Going up"),
    Direction::Down => println!("Going down"),
    Direction::Left => println!("Going left"),
    Direction::Right => println!("Going right"),
  }
}
```

## Standard Library

### Input

For String input

```rust
use std::io;
fn main() {
  let mut input = String::new();
  io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");
  println!("You entered: {}", input.trim_end());
}
```

For Integer Input

```rust
use std::io;
use rand::prelude::*;
fn main() {
  let mut guess = String::new();
  io::stdin()
     .read_line(&mut guess)
     .expect("Failed to read line");
  let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
  };
}
```

### Output

#### Key Formatting Options

- **Default Display (`{}`)**: Prints the shortest representation that round-trips to the original value (usually up to 6 digits for `f32`, more for `f64`).

- **Fixed Precision (`{:.n}`)**: Forces `n` digits after decimal point, last digit rounded.

- **Scientific Notation (`{:e}` or `{:E}`)**: Output in exponential form (e.g., `1.23e4`).

- **All Available Precision (`{:.}`)**: Using a dot with no number (e.g., `{:.}`)

- **Padding and Alignment**: You can combine width and alignment flags, such as `{:>10.2}` for right-aligned, 10-character wide, 2-decimal output.
