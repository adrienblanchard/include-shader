# include-shader

A Rust macro for including shader files as string with [dependencies]() support.

## Setup

Although this library works on `stable`, your shader files changes might not be detected because of caching. Therefore, until [`track_path`](https://doc.rust-lang.org/stable/proc_macro/tracked_path/fn.path.html) API stabilizes, it is recommended to use `nightly` so your shader files are tracked.

### Nightly toolchain

For the best experience (shader files tracking), `nightly` is required.

Add the following to your `Cargo.toml` manifest file:

```toml
[dependencies]
include-shader = { version = "0.1.0", features = ["nightly"] }
```

### Stable toolchain

Add the following to your `Cargo.toml` manifest file:

```toml
[dependencies]
include-shader = "0.1.0"
```

## Example

```rust
use include_shader::include_shader;

fn main() {
   // ...
   let frag_shader = compile_shader(
       &context,
       WebGl2RenderingContext::FRAGMENT_SHADER,
       include_shader!("src/shaders/fragment_shader.glsl"),
   )?;
   // ...
}
```

## Documentation

For more details on how to use this macro, see the [documentation (docs.rs)]().

## License

Distributed under the terms of both the [MIT license](LICENSE-MIT) and the [Apache License (Version 2.0)](LICENSE-APACHE).

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.