use include_shader::include_shader;

#[test]
fn includes_empty_file() {
    let shader = include_shader!("tests/shaders/empty.glsl");

    assert_eq!(shader, "");
}

#[test]
fn includes_file_with_no_include() {
    let shader = include_shader!("tests/shaders/no_include.glsl");

    assert_eq!(shader, include_str!("shaders/no_include.glsl"));
}

#[test]
fn includes_file_with_includes() {
    let shader = include_shader!("tests/shaders/includes.glsl");

    assert!(!shader.contains(r#"#include "./tests/shaders/functions/luminance.glsl"#));
    assert!(!shader.contains(r#"#include "./tests/shaders/functions/rand.glsl""#));
    assert!(shader.contains(include_str!("shaders/functions/luminance.glsl")));
    assert!(shader.contains(include_str!("shaders/functions/rand.glsl")));
}
