//! A library to help working with shaders.
//!
//! Although this library works on `stable`, your shader files changes might not be detected because
//! of caching. Therefore, until
//! [`track_path`](https://doc.rust-lang.org/stable/proc_macro/tracked_path/fn.path.html)
//! API stabilizes, it is recommended to use the `nightly` toolchain and feature flag
//! so your shader files are tracked.
//!
//! ## Optional features
//! **`nightly`** - Enables nightly APIs like
//! [`track_path`](https://doc.rust-lang.org/stable/proc_macro/tracked_path/fn.path.html)
//! for shader files tracking.

#![cfg_attr(feature = "nightly", feature(track_path))]

mod dependency_graph;

use dependency_graph::DependencyGraph;
use lazy_static::lazy_static;
use proc_macro::{Literal, TokenStream, TokenTree};
use regex::Regex;
use std::fs::{canonicalize, read_to_string};
use std::path::{Path, PathBuf};

fn resolve_path(path: &str) -> PathBuf {
    canonicalize(&path).unwrap_or_else(|e| {
        panic!(
            "An error occured while trying to resolve path: {:?}. Error: {}",
            path, e
        )
    })
}

fn track_file(_path: &Path) {
    #[cfg(feature = "nightly")]
    proc_macro::tracked_path::path(_path.to_string_lossy());
}

fn process_file(path: &Path, dependency_graph: &mut DependencyGraph) -> String {
    let content = read_to_string(path).unwrap_or_else(|e| {
        panic!(
            "An error occured while trying to read file: {}. Error: {}",
            path.to_string_lossy(),
            e
        )
    });

    track_file(path);

    process_includes(path, content, dependency_graph)
}

fn process_includes(
    source_path: &Path,
    source_file_content: String,
    dependency_graph: &mut DependencyGraph,
) -> String {
    lazy_static! {
        static ref INCLUDE_RE: Regex = Regex::new(r#"#include\s+"(?P<file>.*)""#).unwrap();
    }
    let mut result = source_file_content;

    while let Some(captures) = INCLUDE_RE.captures(&result.clone()) {
        let capture = captures.get(0).unwrap();
        let include_path = resolve_path(captures.name("file").unwrap().as_str());

        dependency_graph.add_edge(
            source_path.to_string_lossy().to_string(),
            include_path.to_string_lossy().to_string(),
        );

        if let Some(cycle) = dependency_graph.find_cycle() {
            panic!("Circular dependency detected: {}", cycle.join(" -> "));
        }

        result.replace_range(
            capture.start()..capture.end(),
            &process_file(&include_path, dependency_graph),
        );
    }

    result
}

fn unwrap_string_literal(lit: &Literal) -> String {
    let mut repr = lit.to_string();

    repr.remove(0);
    repr.pop();

    repr
}

/// Includes a shader file as a string with dependencies support.
///
/// The file is located relative to the workspace root directory.
///
/// # Panics
///
/// Panics if:
/// * A file specified cannot be found
/// * A circular dependency is detected
///
/// # Examples
///
/// ```ignore
/// use include_shader::include_shader;
///
/// fn main() {
///    // ...
///    let frag_shader = compile_shader(
///        &context,
///        WebGl2RenderingContext::FRAGMENT_SHADER,
///        include_shader!("src/shaders/fragment_shader.glsl"),
///    )?;
///    // ...
/// }
/// ```
///
/// ## Dependencies
///
/// Dependencies are supported within shader files using the `#include` preprocessor directive.
///
/// `rand.glsl`:
///
/// ```glsl
/// float rand(vec2 co) {
///     return fract(sin(dot(co, vec2(12.9898, 78.233))) * 43758.5453);
/// }
/// ```
///
/// `fragment_shader.glsl`:
///
/// ```glsl
/// uniform vec2 u_resolution;
///
/// #include "./src/shaders/functions/rand.glsl"
///
/// void main() {
///    vec2 st = gl_FragCoord.xy / u_resolution.xy;
///
///    gl_FragColor = vec4(vec3(rand(st)), 1.0);
/// }
/// ```
#[proc_macro]
pub fn include_shader(input: TokenStream) -> TokenStream {
    let tokens: Vec<_> = input.into_iter().collect();
    let arg = match tokens.as_slice() {
        [TokenTree::Literal(lit)] => unwrap_string_literal(lit),
        _ => panic!("Takes 1 argument and the argument must be a string literal"),
    };
    let root_path = resolve_path(&arg);
    let mut dependency_graph = DependencyGraph::new();
    let result = process_file(&root_path, &mut dependency_graph);

    format!("{:?}", result).parse().unwrap()
}
