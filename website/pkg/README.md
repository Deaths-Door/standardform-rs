# standardform-rs

Effortlessly operate on numbers like 2e19 or 2*10^4 and more with this Rust implementation of standard form. Simplify, convert, and manipulate large numerical expressions with ease.

## Features

- Create and manipulate numbers in standard form.
- Perform arithmetic operations on numbers with different exponents.
- Easily compare numbers in standard form.
- Clone and debug derive implementations for `StandardForm` struct.
- Intergate it with `num_traits` crate as well.
- Standardform form can be hashed as well using `hash` feature.
- Intergate with nom using `nom` feature
- `no-std` availiable as well
- Bindings to other languages avaliable (enable 'lang' feature to use bindings)
## Usage

Add this library to your `Cargo.toml`:

```toml
[dependencies]
standardform = "0.1.1" # Version  
```
To enable intergation it with `num_traits` , enable `num` feature:

```toml
[dependencies]
standardform = { version = "0.1.1" , features = ["num"] }
```

To enable hashing , enable `hash` feature:

```toml
[dependencies]
standardform = { version = "0.1.1" , features = ["hash"] }
```
```
To enable intergation it with `nom` , enable `nom` feature:

```toml
[dependencies]
standardform = { version = "0.1.1" , features = ["nom"] }
```

Then

```rust
use standardform::StandardForm;

fn main() {
    // Create a new StandardForm instance
    let number = StandardForm::new(2.0, 19);

    // Perform arithmetic operations
    let result = number * StandardForm::new(1.5, -3);

    // Compare numbers
    if result > StandardForm::new(1.0, 10) {
        println!("Result is greater than 1*10^108.");
    }
}
```

Please note that the examples provided here are simplified and serve as a starting point. For comprehensive documentation of the crate, please visit the [crate documentation](https://docs.rs/standardform) for a better understanding of the crate's functionalities and APIs.

## Contributing

Contributions are welcome! If you find a bug or have an enhancement in mind, feel free to open an issue or submit a pull request.
