/// Tests for Spacing API improvements
use ::boxen::Spacing;

#[test]
fn test_spacing_uniform() {
    let spacing = Spacing::uniform(2);
    assert_eq!(spacing.top, 2);
    assert_eq!(spacing.right, 2);
    assert_eq!(spacing.bottom, 2);
    assert_eq!(spacing.left, 2);
}

#[test]
fn test_spacing_symmetric() {
    let spacing = Spacing::symmetric(4, 2);
    assert_eq!(spacing.top, 2);
    assert_eq!(spacing.right, 4);
    assert_eq!(spacing.bottom, 2);
    assert_eq!(spacing.left, 4);
}

#[test]
fn test_spacing_terminal_balanced() {
    let spacing = Spacing::terminal_balanced(2);
    assert_eq!(spacing.top, 2);
    assert_eq!(spacing.right, 6); // 3x horizontal
    assert_eq!(spacing.bottom, 2);
    assert_eq!(spacing.left, 6); // 3x horizontal
}

#[test]
fn test_spacing_terminal_balanced_zero() {
    let spacing = Spacing::terminal_balanced(0);
    assert_eq!(spacing.top, 0);
    assert_eq!(spacing.right, 0);
    assert_eq!(spacing.bottom, 0);
    assert_eq!(spacing.left, 0);
}

#[test]
fn test_spacing_terminal_balanced_large_value() {
    let spacing = Spacing::terminal_balanced(5);
    assert_eq!(spacing.top, 5);
    assert_eq!(spacing.right, 15); // 3x horizontal
    assert_eq!(spacing.bottom, 5);
    assert_eq!(spacing.left, 15); // 3x horizontal
}

#[test]
#[allow(deprecated)]
fn test_spacing_from_usize_deprecated_still_works() {
    // Test that the deprecated From<usize> impl still works
    let spacing = Spacing::from(2);
    assert_eq!(spacing.top, 2);
    assert_eq!(spacing.right, 6); // 3x horizontal
    assert_eq!(spacing.bottom, 2);
    assert_eq!(spacing.left, 6); // 3x horizontal
}

#[test]
#[allow(deprecated)]
fn test_spacing_from_usize_matches_terminal_balanced() {
    // Verify that From<usize> and terminal_balanced produce the same result
    let from_impl = Spacing::from(3);
    let terminal_balanced = Spacing::terminal_balanced(3);

    assert_eq!(from_impl.top, terminal_balanced.top);
    assert_eq!(from_impl.right, terminal_balanced.right);
    assert_eq!(from_impl.bottom, terminal_balanced.bottom);
    assert_eq!(from_impl.left, terminal_balanced.left);
}

#[test]
fn test_spacing_uniform_vs_terminal_balanced() {
    let uniform = Spacing::uniform(2);
    let terminal = Spacing::terminal_balanced(2);

    // Uniform should have equal spacing
    assert_eq!(uniform.top, uniform.right);
    assert_eq!(uniform.top, uniform.bottom);
    assert_eq!(uniform.top, uniform.left);

    // Terminal balanced should have 3x horizontal
    assert_eq!(terminal.top, 2);
    assert_eq!(terminal.right, 6);
    assert_eq!(terminal.bottom, 2);
    assert_eq!(terminal.left, 6);

    // They should be different
    assert_ne!(uniform.right, terminal.right);
}

#[test]
fn test_spacing_constructors_with_builder() {
    use ::boxen::builder;

    // Test that new constructors work with the builder
    let result1 = builder()
        .padding(Spacing::uniform(2))
        .render("Test")
        .unwrap();
    assert!(result1.contains("Test"));

    let result2 = builder()
        .padding(Spacing::terminal_balanced(1))
        .render("Test")
        .unwrap();
    assert!(result2.contains("Test"));

    let result3 = builder()
        .padding(Spacing::symmetric(4, 2))
        .render("Test")
        .unwrap();
    assert!(result3.contains("Test"));
}

#[test]
fn test_spacing_horizontal_vertical_methods() {
    let spacing = Spacing::terminal_balanced(2);
    assert_eq!(spacing.horizontal(), 12); // left + right = 6 + 6
    assert_eq!(spacing.vertical(), 4); // top + bottom = 2 + 2
}

#[test]
fn test_spacing_is_empty() {
    let empty = Spacing::uniform(0);
    assert!(empty.is_empty());

    let non_empty = Spacing::uniform(1);
    assert!(!non_empty.is_empty());

    let terminal = Spacing::terminal_balanced(0);
    assert!(terminal.is_empty());
}
