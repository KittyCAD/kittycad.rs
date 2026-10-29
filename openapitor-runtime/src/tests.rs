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

#[test]
fn test_json_error_limits_context_window() {
    let input = concat!(
        "[\n  \"one\",\n  \"two\",\n  \"three\",\n  \"four\",\n  42,\n",
        "  \"six\",\n  \"seven\",\n  \"eight\",\n  \"nine\"\n]",
    );
    for (context_lines, expected) in [
        (
            0,
            concat!(
                "\n 6 | 42,\n",
                "   |   ^ invalid type: integer `42`, expected a string at line 6 column 4\n",
            ),
        ),
        (
            1,
            concat!(
                "\n   | \"four\",\n 6 | 42,\n",
                "   |   ^ invalid type: integer `42`, expected a string at line 6 column 4\n",
                "   | \"six\",\n",
            ),
        ),
        (
            3,
            concat!(
                "\n   | \"two\",\n   | \"three\",\n   | \"four\",\n 6 | 42,\n",
                "   |   ^ invalid type: integer `42`, expected a string at line 6 column 4\n",
                "   | \"six\",\n   | \"seven\",\n   | \"eight\",\n",
            ),
        ),
    ] {
        let mut error = json_response_error(input);
        error.set_context_lines(context_lines);
        assert_eq!(
            plain_diagnostic(&error),
            expected,
            "context_lines={context_lines}",
        );
    }
}

#[test]
fn test_json_error_context_window_at_start() {
    let input = concat!(
        "[42,\n  \"one\",\n  \"two\",\n  \"three\",\n",
        "  \"four\",\n  \"five\",\n  \"six\",\n  \"seven\"\n]",
    );
    let mut error = json_response_error(input);
    error.set_context_lines(1);
    // The previous formatter fills the window with following lines near the start.
    assert_eq!(
        plain_diagnostic(&error),
        concat!(
            "\n 1 | [42,\n",
            "   |    ^ invalid type: integer `42`, expected a string at line 1 column 3\n",
            "   |   \"one\",\n   |   \"two\",\n",
        ),
    );
}

#[test]
fn test_json_error_context_window_at_end_with_multidigit_line_number() {
    let input = concat!(
        "[\n  \"one\",\n  \"two\",\n  \"three\",\n  \"four\",\n",
        "  \"five\",\n  \"six\",\n  \"seven\",\n  \"eight\",\n  \"nine\",\n  42]",
    );
    let mut error = json_response_error(input);
    error.set_context_lines(1);
    assert_eq!(
        plain_diagnostic(&error),
        concat!(
            "\n    | \"nine\",\n 11 | 42]\n",
            "    |   ^ invalid type: integer `42`, expected a string at line 11 column 4\n",
        ),
    );
}

#[test]
fn test_json_error_removes_only_common_indentation() {
    let input = "  [\n    \"first\",\n    42,\n      \"last\"\n  ]";
    assert_eq!(
        plain_diagnostic(&json_response_error(input)),
        concat!(
            "\n   | [\n   |   \"first\",\n 3 |   42,\n",
            "   |     ^ invalid type: integer `42`, expected a string at line 3 column 6\n",
            "   |     \"last\"\n   | ]\n",
        ),
    );
}

#[test]
fn test_json_error_preserves_blank_context_lines() {
    let input = "  [\n    \"first\",\n\n    42,\n  ]";
    assert_eq!(
        plain_diagnostic(&json_response_error(input)),
        concat!(
            "\n   |   [\n   |     \"first\",\n   | \n 4 |     42,\n",
            "   |       ^ invalid type: integer `42`, expected a string at line 4 column 6\n",
            "   |   ]\n",
        ),
    );
}

#[test]
fn test_json_error_normalizes_tabs() {
    for (input, column) in [
        ("\t[\n\t\t\"first\",\n\t\t42,\n\t]", 4),
        ("[\n\t\"first\",\n\t42,\n]", 3),
    ] {
        assert_eq!(
            plain_diagnostic(&json_response_error(input)),
            format!(
                concat!(
                    "\n   | [\n   |  \"first\",\n 3 |  42,\n",
                    "   |    ^ invalid type: integer `42`, expected a string at line 3 column {}\n",
                    "   | ]\n",
                ),
                column,
            ),
            "input: {input:?}",
        );
    }
}

#[test]
fn test_json_error_handles_lf_and_crlf() {
    for newline in ["\n", "\r\n"] {
        let input = format!("[{newline}  \"first\",{newline}  42{newline}]{newline}");
        assert_eq!(
            plain_diagnostic(&json_response_error(&input)),
            concat!(
                "\n   | [\n   |   \"first\",\n 3 |   42\n",
                "   |     ^ invalid type: integer `42`, expected a string at line 3 column 4\n",
                "   | ]\n",
            ),
            "newline: {newline:?}",
        );
    }
}

#[test]
fn test_json_error_without_context_omits_neighbors_and_keeps_long_line() {
    let input = "[\n  \"before\",\n  42, \"abcdefghijklmnopqrstuvwxyz0123456789\"\n]";
    let mut error = json_response_error(input);
    error
        .set_context_lines(1)
        .set_context_characters(1)
        .set_contextualize(false);
    assert_eq!(
        plain_diagnostic(&error),
        concat!(
            "\n 3 |   42, \"abcdefghijklmnopqrstuvwxyz0123456789\"\n",
            "   |     ^ invalid type: integer `42`, expected a string at line 3 column 4\n",
        ),
    );
}

#[test]
fn test_json_error_settings_are_per_instance_and_can_be_restored() {
    let input = "[\n  \"first\",\n  42,\n  \"last\"\n]";
    let mut error = json_response_error(input);
    let other = json_response_error(input);
    let original = plain_diagnostic(&other);
    error
        .set_context_lines(0)
        .set_context_characters(1)
        .set_contextualize(false);
    assert_ne!(plain_diagnostic(&error), original);
    assert_eq!(plain_diagnostic(&other), original);
    assert!(other.get_contextualize());
    assert_eq!(other.get_context_lines(), 3);
    assert_eq!(other.get_context_characters(), 30);

    error
        .set_context_lines(3)
        .set_context_characters(30)
        .set_contextualize(true);
    assert!(error.get_contextualize());
    assert_eq!(error.get_context_lines(), 3);
    assert_eq!(error.get_context_characters(), 30);
    assert_eq!(plain_diagnostic(&error), original);
}

#[test]
fn test_json_error_clipping_boundaries() {
    for (input, expected) in [
        (
            "[42]",
            concat!(
                "\n 1 | [42]\n",
                "   |    ^ invalid type: integer `42`, expected a string at line 1 column 3\n",
            ),
        ),
        (
            "[42, 0]",
            concat!(
                "\n 1 | [42, 0]\n",
                "   |    ^ invalid type: integer `42`, expected a string at line 1 column 3\n",
            ),
        ),
        (
            "[42, 10]",
            concat!(
                "\n 1 | [42, 10...\n",
                "   |    ^ invalid type: integer `42`, expected a string at line 1 column 3\n",
            ),
        ),
        (
            "[42, \"abcdefghijklmnopqrstuvwxyz\"]",
            concat!(
                "\n 1 | [42, \"a...\n",
                "   |    ^ invalid type: integer `42`, expected a string at line 1 column 3\n",
            ),
        ),
        (
            "[ 42, \"abcdef\"]",
            concat!(
                "\n 1 | [ 42, \"...\n",
                "   |     ^ invalid type: integer `42`, expected a string at line 1 column 4\n",
            ),
        ),
        (
            "[\"abcdefghijklmnopqrstuvwxyz\", 42]",
            concat!(
                "\n 1 | ..., 42]\n",
                "   |        ^ invalid type: integer `42`, expected a string at line 1 column 33\n",
            ),
        ),
    ] {
        let mut error = json_response_error(input);
        error.set_context_characters(3);
        assert_eq!(plain_diagnostic(&error), expected, "input: {input:?}");
    }
}

#[test]
fn test_json_error_clips_long_lines_by_default() {
    let input = format!("[42, \"{}\"]", "a".repeat(80));
    assert_eq!(
        plain_diagnostic(&json_response_error(&input)),
        format!(
            "\n 1 | [42, \"{}...\n   |    ^ invalid type: integer `42`, expected a string at line \
             1 column 3\n",
            "a".repeat(55),
        ),
    );
}

#[test]
fn test_json_error_zero_context_characters() {
    let input = "[\"abc\", 42, \"after\"]";
    let mut error = json_response_error(input);
    error.set_context_characters(0);
    assert_eq!(
        plain_diagnostic(&error),
        concat!(
            "\n 1 | ...2...\n",
            "   |     ^ invalid type: integer `42`, expected a string at line 1 column 10\n",
        ),
    );
}

#[test]
fn test_json_error_clips_multibyte_context_characters() {
    let input = "[42, \"\u{e9}\u{e9}\u{e9}\u{e9}\u{e9}\u{e9}\"]";
    let mut error = json_response_error(input);
    error.set_context_characters(4);
    assert_eq!(
        plain_diagnostic(&error),
        concat!(
            "\n 1 | [42, \"\u{e9}\u{e9}\u{e9}...\n",
            "   |    ^ invalid type: integer `42`, expected a string at line 1 column 3\n",
        ),
    );
}

#[test]
fn test_json_error_formats_syntax_eof_and_trailing_character_errors() {
    for (input, expected) in [
        (
            "[\"ok\",]",
            "\n 1 | [\"ok\",]\n   |        ^ trailing comma at line 1 column 7\n",
        ),
        (
            "[] extra",
            "\n 1 | [] extra\n   |     ^ trailing characters at line 1 column 4\n",
        ),
        (
            "[\"ok\"",
            "\n 1 | [\"ok\"\n   |      ^ EOF while parsing a list at line 1 column 5\n",
        ),
        (
            "{",
            "\n 1 | {\n   |  ^ EOF while parsing an object at line 1 column 1\n",
        ),
    ] {
        let source = serde_json::from_str::<serde_json::Value>(input).unwrap_err();
        let error = crate::SerdeError::new(input.to_string(), source);
        assert_eq!(plain_diagnostic(&error), expected, "input: {input:?}");
    }
}

#[test]
fn test_json_error_formats_multiline_syntax_error() {
    let input = "{\n  \"key\": ]\n}";
    let source = serde_json::from_str::<serde_json::Value>(input).unwrap_err();
    let error = crate::SerdeError::new(input.to_string(), source);
    assert_eq!(
        plain_diagnostic(&error),
        concat!(
            "\n   | {\n 2 |   \"key\": ]\n",
            "   |           ^ expected value at line 2 column 10\n   | }\n",
        ),
    );
}

#[test]
fn test_json_error_formats_whitespace_only_response() {
    assert_eq!(
        plain_diagnostic(&json_response_error("   ")),
        "\n 1 | \n   | ^ EOF while parsing a value at line 1 column 3\n",
    );
}

#[test]
fn test_json_error_without_response_body_preserves_source_location() {
    let source = serde_json::from_str::<serde_json::Value>("{\n  \"key\": ]\n}").unwrap_err();
    let error = crate::SerdeError::new(String::new(), source);
    assert_eq!(
        plain_diagnostic(&error),
        "expected value at line 2 column 10\n",
    );
}

#[test]
fn test_json_error_saturating_context_settings_preserve_full_diagnostic() {
    let input = "{\n  \"hello\": ]\n}";
    for (lines, characters) in [(usize::MAX, 30), (3, usize::MAX), (usize::MAX, usize::MAX)] {
        let source = serde_json::from_str::<serde_json::Value>(input).unwrap_err();
        let mut error = crate::SerdeError::new(input.to_string(), source);
        error
            .set_context_lines(lines)
            .set_context_characters(characters);
        assert_eq!(
            plain_diagnostic(&error),
            concat!(
                "\n   | {\n 2 |   \"hello\": ]\n",
                "   |             ^ expected value at line 2 column 12\n   | }\n",
            ),
            "context_lines={lines}, context_characters={characters}",
        );
    }
}

#[test]
fn test_serde_error_supports_standard_error_trait_objects() {
    let error: Box<dyn std::error::Error + Send + Sync> = Box::new(json_response_error(""));
    assert!(error.downcast_ref::<crate::SerdeError>().is_some());
    assert!(error.source().is_none());
    assert_eq!(
        plain_diagnostic(&error),
        "EOF while parsing a value at line 1 column 0\n",
    );
}

#[test]
fn test_json_error_propagates_formatter_errors() {
    struct LimitedWriter(usize);

    impl std::fmt::Write for LimitedWriter {
        fn write_str(&mut self, text: &str) -> std::fmt::Result {
            self.0 = self.0.checked_sub(text.len()).ok_or(std::fmt::Error)?;
            Ok(())
        }
    }

    for input in [
        "",
        "[\n  \"abcdefghijklmnopqrstuvwxyz0123456789\", 42, \"after\"\n]",
    ] {
        let mut error = json_response_error(input);
        error.set_context_characters(3);
        for limit in 0..error.to_string().len() {
            assert!(
                std::fmt::write(&mut LimitedWriter(limit), format_args!("{error}")).is_err(),
                "input: {input:?}, writer limit: {limit}",
            );
        }
    }
}

#[test]
fn test_json_error_retains_message_at_eof_after_newline() {
    for input in ["\n", "[\n", "[\n\n", "\r\n", "[\r\n", "[\r\n\r\n"] {
        let source = serde_json::from_str::<Vec<String>>(input).unwrap_err();
        let message = source.to_string();
        let mut error = crate::SerdeError::new(input.to_string(), source);
        for context_lines in [0, 3, usize::MAX] {
            for contextualize in [true, false] {
                error
                    .set_context_lines(context_lines)
                    .set_contextualize(contextualize);
                assert_eq!(
                    plain_diagnostic(&error),
                    format!("{message}\n"),
                    "input: {input:?}, context_lines={context_lines}, \
                     contextualize={contextualize}",
                );
            }
        }
    }
}

#[test]
fn test_json_error_retains_message_without_source_location() {
    let source = serde_json::from_value::<Vec<String>>(serde_json::json!([42])).unwrap_err();
    assert_eq!((source.line(), source.column()), (0, 0));
    let message = source.to_string();
    let mut error = crate::SerdeError::new("[42]".to_string(), source);
    for contextualize in [true, false] {
        error.set_contextualize(contextualize);
        assert_eq!(plain_diagnostic(&error), format!("{message}\n"));
    }
}

#[test]
fn test_json_error_clipping_keeps_indented_offending_value() {
    let mut error = json_response_error("[\n        42, \"after\"\n]");
    error.set_context_lines(0).set_context_characters(1);
    assert_eq!(
        plain_diagnostic(&error),
        concat!(
            "\n 2 | 42,...\n",
            "   |   ^ invalid type: integer `42`, expected a string at line 2 column 10\n",
        ),
    );
}

#[test]
fn test_json_error_clipping_keeps_offending_value_after_unicode() {
    for (input, column) in [
        ("[\"\u{e9}\", 42]", 9),
        ("[\"e\u{301}\", 42]", 10),
        ("[\"\u{1f468}\u{200d}\u{1f469}\u{200d}\u{1f466}\", 42]", 25),
    ] {
        let mut error = json_response_error(input);
        error.set_context_characters(3);
        assert_eq!(
            plain_diagnostic(&error),
            format!(
                concat!(
                    "\n 1 | ..., 42]\n",
                    "   |        ^ invalid type: integer `42`, expected a string at line 1 column \
                     {}\n",
                ),
                column,
            ),
            "input: {input:?}",
        );
    }
}

#[test]
fn test_json_error_clipping_handles_indentation_and_unicode_together() {
    let input = "[\n        \"\u{1f468}\u{200d}\u{1f469}\u{200d}\u{1f466}\", 42, \"after\"\n]";
    let mut error = json_response_error(input);
    error.set_context_lines(0).set_context_characters(3);
    assert_eq!(
        plain_diagnostic(&error),
        concat!(
            "\n 2 | ..., 42, \"...\n",
            "   |        ^ invalid type: integer `42`, expected a string at line 2 column 32\n",
        ),
    );
}

#[test]
fn test_json_error_column_inside_multibyte_character() {
    let input = "[\u{e9}]";
    let source = serde_json::from_str::<serde_json::Value>(input).unwrap_err();
    assert_eq!((source.line(), source.column()), (1, 2));
    let mut error = crate::SerdeError::new(input.to_string(), source);
    error.set_context_characters(0);
    assert_eq!(
        plain_diagnostic(&error),
        "\n 1 | ...\u{e9}...\n   |     ^ expected value at line 1 column 2\n",
    );
}
