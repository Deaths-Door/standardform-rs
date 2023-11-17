# standardform-rs

Effortlessly operate on numbers like 2e19 or 2*10^4 and more with this Rust implementation of standard form. Simplify, convert, and manipulate large numerical expressions with ease.

## Features

- Create and manipulate numbers in standard form.
- Perform arithmetic operations on numbers with different exponents.
- Easily compare numbers in standard form.
- Clone and debug derive implementations for `StandardForm` struct.
- Integrate it with `num_traits` crate as well.
- Standardform can be hashed as well using `hash` feature.
- Integrate with nom using `nom` feature.
- `no-std` available as well.
- Bindings to other languages available (enable 'lang name'` feature to use bindings).

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
Additionally, you can explore a [website](/website/index.html) showcasing a subset of its features.

## JavaScript Bindings

If you prefer not to build the JavaScript bindings yourself, you can download a pre-built version of the `pkg` directory from the [releases page](#insert-link-to-releases-page). Follow these instructions to use the pre-built 'js' bindings:

1. Download the pre-built `pkg` directory from the [releases page](#insert-link-to-releases-page).

2. Extract the contents of the downloaded archive.

3. Ensure your HTML file references the pre-built JavaScript file. Add the following script tag to your HTML file:

    ```html
    <script type="module">
        import init , { StandardForm } from '/path-to-pkg/standardform.js';

        await init()
    </script>
    ```

    Replace `'/path-to-pkg/standardform.js'` with the correct path to the downloaded 'standardform.js' file.

4. Your library is now ready to be used in your JavaScript code.

**In case** you are using `webpack` , follow this [guide](https://rustwasm.github.io/docs/wasm-bindgen/examples/hello-world.html#webpack-specific-files)

Here are the [guides](https://rustwasm.github.io/docs/wasm-bindgen/reference/deployment.html) to use this code with different methods of deployment and integration


Note: While using pre-built bindings is convenient, it's recommended to build the bindings yourself if you plan on making changes to the Rust code, as explained below

## JavaScript Integration

For seamless integration between Rust and JavaScript, this library supports WebAssembly (Wasm). Follow these steps to update bindings when the Rust code changes:

1. Ensure you have `wasm-pack` installed:

    ```bash
    cargo install wasm-pack
    ```

2. Build the Wasm package:

    ```bash
    wasm-pack build --target web --out-dir ./website
    ```

3. If you make changes to your Rust code, rebuild the Wasm package to update the bindings

This ensures that your JavaScript bindings stay synchronized with any changes made to your Rust code. Remember to update the bindings whenever you modify your Rust implementation to maintain a consistent integration between the two languages.


## Contributing

Contributions are welcome! If you find a bug or have an enhancement in mind, feel free to open an issue or submit a pull request.
