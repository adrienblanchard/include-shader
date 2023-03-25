# include-shader

A Rust macro for including shader files as string with [dependencies](https://docs.rs/include-shader/latest/include_shader/macro.include_shader.html#dependencies) support.

## Setup

Although this library works on `stable`, detection of shader file changes is not guaranteed due to caching. Therefore, it is recommended to use `nightly` 
along with the `track-path` feature enabled until the [`track_path`](https://doc.rust-lang.org/stable/proc_macro/tracked_path/fn.path.html) API stabilizes.

### Nightly toolchain (recommended)

For the best experience, use `nightly` to gain access to extra [features](https://docs.rs/include-shader/latest/include_shader/index.html#optional-features):

- File tracking
- Relative path resolution

Add the following to your `Cargo.toml` manifest file:

```toml
[dependencies]
include-shader = { version = "0.2.0", features = ["track-file", "relative-path"] }
```

### Stable toolchain

Add the following to your `Cargo.toml` manifest file:

```toml
[dependencies]
include-shader = "0.2.0"
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

For more details on how to use this macro, see the [documentation](https://docs.rs/include-shader/latest/include_shader/macro.include_shader.html).

## License

Distributed under the terms of both the [MIT license](LICENSE-MIT) and the [Apache License (Version 2.0)](LICENSE-APACHE).

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.