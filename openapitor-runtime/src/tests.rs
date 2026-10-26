fn json_response_error(input: &str) -> crate::SerdeError {
    let error = serde_json::from_str::<Vec<String>>(input).unwrap_err();
    crate::SerdeError::new(input.to_string(), error)
}

fn plain_diagnostic(error: &impl std::fmt::Display) -> String {
    regex::Regex::new(r"\x1b\[[0-9;]*m")
        .unwrap()
        .replace_all(&error.to_string(), "")
        .into_owned()
}

#[test]
fn test_json_error_context() {
    let input = "[\n  \"first\",\n  \"second\",\n  42,\n  \"fourth\"\n]";
    let error = json_response_error(input);
    assert!(error.get_contextualize());
    assert_eq!(error.get_context_lines(), 3);
    assert_eq!(error.get_context_characters(), 30);
    // Keep the diagnostic text produced by the previous formatter.
    let expected = concat!(
        "\n   | [\n   |   \"first\",\n   |   \"second\",\n 4 |   42,\n",
        "   |     ^ invalid type: integer `42`, expected a string at line 4 column 4\n",
        "   |   \"fourth\"\n   | ]\n",
    );
    assert_eq!(plain_diagnostic(&error), expected);
}

#[test]
fn test_json_error_without_response_body() {
    assert_eq!(
        plain_diagnostic(&json_response_error("")),
        "EOF while parsing a value at line 1 column 0\n",
    );
}

#[test]
fn test_json_error_context_settings() {
    let input = "[\n  \"first\",\n  \"second\",\n  42,\n  \"fourth\"\n]";
    let mut error = json_response_error(input);
    error.set_context_lines(0).set_context_characters(3);
    assert_eq!(error.get_context_lines(), 0);
    assert_eq!(error.get_context_characters(), 3);
    assert_eq!(
        plain_diagnostic(&error),
        concat!(
            "\n 4 | 42,\n",
            "   |   ^ invalid type: integer `42`, expected a string at line 4 column 4\n",
        ),
    );
}

#[test]
fn test_json_error_long_line_settings() {
    let input = "[\"abcdefghijklmnopqrstuvwxyz0123456789\", 42, \"after\"]";
    let mut error = json_response_error(input);
    error.set_context_characters(3);
    assert_eq!(
        plain_diagnostic(&error),
        concat!(
            "\n 1 | ..., 42, \"...\n",
            "   |        ^ invalid type: integer `42`, expected a string at line 1 column 43\n",
        ),
    );
    error.set_contextualize(false);
    assert!(!error.get_contextualize());
    assert_eq!(
        plain_diagnostic(&error),
        format!(
            "\n 1 | {input}\n   |                                            ^ invalid type: \
             integer `42`, expected a string at line 1 column 43\n"
        ),
    );
}

#[test]
fn test_json_error_preserves_grapheme_clusters() {
    let input = "[\"abc\", 42, \"e\u{301}\u{1f468}\u{200d}\u{1f469}\u{200d}\u{1f466}xyzmore\"]";
    let mut error = json_response_error(input);
    error.set_context_characters(5);
    assert_eq!(
        plain_diagnostic(&error),
        concat!(
            "\n 1 | ...c\", 42, \"e\u{301}\u{1f468}\u{200d}\u{1f469}\u{200d}\u{1f466}...\n",
            "   |          ^ invalid type: integer `42`, expected a string at line 1 column 10\n",
        ),
    );
}

#[test]
fn test_json_error_handles_extreme_context_settings() {
    let input = "{\n  \"hello\": ]\n}";
    let source = serde_json::from_str::<serde_json::Value>(input).unwrap_err();
    let message = source.to_string();
    let mut error = crate::SerdeError::new(input.to_string(), source);
    error.set_context_lines(0).set_context_characters(0);
    assert!(plain_diagnostic(&error).contains(&format!("^ {message}")));
    error
        .set_context_lines(usize::MAX)
        .set_context_characters(usize::MAX);
    assert!(plain_diagnostic(&error).contains(&format!("^ {message}")));
}
