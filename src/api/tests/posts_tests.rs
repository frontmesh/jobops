use crate::api::posts::*;

#[test]
fn test_extract_between_markers() {
    let input = r#"+++
title = "Test Post"
tags = ["rust"]
+++
# Hello World

This is a **bold** text and *italic* text.

## Section
- List item 1
- List item 2"#;

    let result = extract_between_markers(input, "+++");
    assert!(result.is_some());
    let (frontmatter, html) = result.unwrap();

    // Debug output
    println!("Frontmatter:\n{}", frontmatter);
    println!("HTML:\n{}", html);

    // Check frontmatter
    assert!(frontmatter.contains("title = \"Test Post\""));
    assert!(frontmatter.contains("tags = [\"rust\"]"));

    // Check HTML output
    assert!(html.contains("<h1>Hello World</h1>"));
    assert!(html.contains("<strong>bold</strong>"));
    assert!(html.contains("<em>italic</em>"));
    assert!(html.contains("<h2>Section</h2>"));
    assert!(html.contains("<li>List item 1</li>"));

    // Test case 2: Markdown with code blocks
    let input = r#"+++
title = "Code Example"
+++
Here's some code:

```rust
fn main() {
    println!("Hello, world!");
}
```"#;

    let result = extract_between_markers(input, "+++");
    assert!(result.is_some());
    let (_, html) = result.unwrap();

    // Debug output
    println!("\nCode block HTML:\n{}", html);

    // Update assertions to match actual output
    assert!(html.contains("<code class=\"language-rust\">"));
    assert!(html.contains("println!"));
    assert!(html.contains("<pre><code"));
}

#[test]
fn test_parse_frontmatter_line() {
    let test_cases = vec![
        (
            r#"tags = ["js", "css", "animation"]"#,
            "tags",
            Some(vec![
                "js".to_string(),
                "css".to_string(),
                "animation".to_string(),
            ]),
        ),
        (
            r#"tags = ["js", " css", " animation"]"#, // with leading spaces
            "tags",
            Some(vec![
                "js".to_string(),
                "css".to_string(),
                "animation".to_string(),
            ]),
        ),
        (
            r#"tags = ["CSS/SASS", "js", "css"]"#, // with special characters
            "tags",
            Some(vec![
                "CSS/SASS".to_string(),
                "js".to_string(),
                "css".to_string(),
            ]),
        ),
    ];

    for (input, prefix, expected) in test_cases {
        let result = parse_frontmatter_line(input, prefix);
        println!("Input: {}", input);
        println!("Result: {:?}", result);
        println!("Expected: {:?}", expected);
        assert_eq!(result, expected);
    }
}
