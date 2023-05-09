use crate::codegen::swift::{render_to_strings, RenderIntput};
use crate::manifest::parse_str;

const STRUCT_TEMPLATE: &str = include_str!("../codegen/templates/swift/struct.hbs");
const ENUM_TEMPLATE: &str = include_str!("../codegen/templates/swift/enum.hbs");
const EXTENSION_TEMPLATE: &str = include_str!("../codegen/templates/swift/extension.hbs");
const PROTO_TEMPLATE: &str = include_str!("../codegen/templates/swift/proto.hbs");

/// Convenience function.
fn create_intput(yaml: &str) -> RenderIntput {
    let file_info = parse_str(yaml).unwrap();

    RenderIntput {
        file_info,
        struct_template: STRUCT_TEMPLATE,
        enum_template: ENUM_TEMPLATE,
        extension_template: EXTENSION_TEMPLATE,
        proto_template: PROTO_TEMPLATE,
    }
}

// Convenience function: runs the codegen on the given `input` and compares it
// with the `expected` value. Expects a single, rendered file as output.
fn render_and_compare_struct(input: &str, expected: &str) {
    let input = create_intput(input);
    let rendered = render_to_strings(input).unwrap();

    assert_eq!(rendered.structs.len(), 1);
    assert!(rendered.enums.is_empty());
    assert!(rendered.extensions.is_empty());
    assert!(rendered.protos.is_empty());

    let (_name, output) = &rendered.structs[0];
    println!("{output}");
    assert_eq!(output, expected);
}

fn render_and_compare_enum(input: &str, expected: &str) {
    let input = create_intput(input);
    let rendered = render_to_strings(input).unwrap();

    assert!(rendered.structs.is_empty());
    assert_eq!(rendered.enums.len(), 1);
    assert!(rendered.extensions.is_empty());
    assert!(rendered.protos.is_empty());

    let (_name, output) = &rendered.enums[0];
    assert_eq!(output, expected);
}

#[test]
fn single_struct() {
    const INPUT: &str = include_str!("samples/struct.input.yaml");
    const EXPECTED: &str = include_str!("samples/struct.output.swift");

    render_and_compare_struct(INPUT, EXPECTED);
}

#[test]
fn single_class() {
    const INPUT: &str = include_str!("samples/class.input.yaml");
    const EXPECTED: &str = include_str!("samples/class.output.swift");

    render_and_compare_struct(INPUT, EXPECTED);
}

#[test]
fn private() {
    const INPUT: &str = include_str!("samples/private_class.input.yaml");
    const EXPECTED: &str = include_str!("samples/private_class.output.swift");

    render_and_compare_struct(INPUT, EXPECTED);
}

#[test]
fn optional() {
    const INPUT: &str = include_str!("samples/optional.input.yaml");
    const EXPECTED: &str = include_str!("samples/optional.output.swift");

    render_and_compare_struct(INPUT, EXPECTED);
}

#[test]
fn enum_with_description() {
    const INPUT: &str = include_str!("samples/enum.input.yaml");
    const EXPECTED: &str = include_str!("samples/enum.output.swift");

    render_and_compare_enum(INPUT, EXPECTED);
}

#[test]
fn privat_enum_with_description() {
    const INPUT: &str = include_str!("samples/enum_private.input.yaml");
    const EXPECTED: &str = include_str!("samples/enum_private.output.swift");

    render_and_compare_enum(INPUT, EXPECTED);
}

#[test]
fn enum_with_extension() {
    const INPUT: &str = include_str!("samples/enum_extension.input.yaml");
    const EXPECTED_ENUM: &str = include_str!("samples/enum.output.swift");
    const EXPECTED_EXTENSION: &str = include_str!("samples/enum_extension.output.swift");

    let input = create_intput(INPUT);
    let rendered = render_to_strings(input).unwrap();

    assert!(rendered.structs.is_empty());
    assert_eq!(rendered.enums.len(), 1);
    assert_eq!(rendered.extensions.len(), 1);
    assert!(rendered.protos.is_empty());

    // Check generated enum.
    let (_name, output) = &rendered.enums[0];
    assert_eq!(output, EXPECTED_ENUM);

    // Check generated extension.
    let (_name, output) = &rendered.extensions[0];
    assert_eq!(output, EXPECTED_EXTENSION);
}


#[test]
fn non_associated() {
    const INPUT: &str = include_str!("samples/non-associated.input.yaml");
    const EXPECTED: &str = include_str!("samples/non-associated.output.swift");

    render_and_compare_struct(INPUT, EXPECTED);
}