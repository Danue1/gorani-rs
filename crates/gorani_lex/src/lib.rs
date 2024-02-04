use gorani_syntax::SyntaxKind;
use std::borrow::Cow;

pub fn lex(mut source: &str) -> impl Iterator<Item = (SyntaxKind, usize, Cow<str>)> {
    std::iter::from_fn(move || {
        let c = source.chars().next()?;
        let (kind, len, token) = lex_token(c, source);
        let token = token
            .map(Cow::Owned)
            .unwrap_or_else(|| Cow::Borrowed(&source[..len]));
        source = &source[len..];

        Some((kind, len, token))
    })
}

fn lex_token(c: char, source: &str) -> (SyntaxKind, usize, Option<String>) {
    match c {
        '(' => (SyntaxKind::SYMBOL_LEFT_PARENS, 1, None),
        ')' => (SyntaxKind::SYMBOL_RIGHT_PARENS, 1, None),
        ':' => (SyntaxKind::SYMBOL_COLON, 1, None),
        '=' => (SyntaxKind::SYMBOL_EQUAL, 1, None),
        '@' => (SyntaxKind::SYMBOL_AT, 1, None),
        '$' => (SyntaxKind::SYMBOL_DOLLAR, 1, None),
        '&' => (SyntaxKind::SYMBOL_AMPERSAND, 1, None),
        '[' => (SyntaxKind::SYMBOL_LEFT_BRACKET, 1, None),
        ']' => (SyntaxKind::SYMBOL_RIGHT_BRACKET, 1, None),
        '{' => (SyntaxKind::SYMBOL_LEFT_BRACE, 1, None),
        '}' => (SyntaxKind::SYMBOL_RIGHT_BRACE, 1, None),
        '!' => (SyntaxKind::SYMBOL_EXCLAMATION, 1, None),
        '|' => (SyntaxKind::SYMBOL_PIPE, 1, None),
        '.' => {
            if source.starts_with("...") {
                (SyntaxKind::SYMBOL_SPREAD, 3, None)
            } else {
                (SyntaxKind::ERROR, 1, None)
            }
        }
        'a'..='z' | 'A'..='Z' | '_' => {
            let len = source
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                .count();

            (SyntaxKind::NAME, len, None)
        }
        '-' => {
            let (kind, len) = number(true, source);

            (kind, len, None)
        }
        '0'..='9' => {
            let (kind, len) = number(false, source);

            (kind, len, None)
        }
        '"' => {
            let (kind, len, token) = if source.starts_with("\"\"\"") {
                block_string(source)
            } else {
                return single_line_string(source);
            };

            (kind, len, Some(token))
        }
        ',' | ' ' | '\t' | '\n' | '\r' => {
            let len = source
                .chars()
                .take_while(|c| matches!(c, ',' | ' ' | '\t' | '\n' | '\r'))
                .count();

            (SyntaxKind::IGNORED_TOKEN, len, None)
        }
        '#' => {
            let line = match source.lines().next() {
                Some(line) => line.len(),
                None => source.len(),
            };

            (SyntaxKind::COMMENT, line, None)
        }
        _ => (SyntaxKind::ERROR, 1, None),
    }
}

fn block_string(_source: &str) -> (SyntaxKind, usize, String) {
    std::todo!();
}

fn single_line_string(source: &str) -> (SyntaxKind, usize, Option<String>) {
    let rest = &source[1..];
    let Some(line) = rest.lines().next() else {
        return (SyntaxKind::ERROR, source.len(), None);
    };

    let mut len = 1;
    let mut string = String::new();
    let mut rest = line.chars();
    while let Some(c) = rest.next() {
        len += 1;

        match c {
            '"' => break,
            '\\' => match rest.next() {
                Some('"') => {
                    len += 1;
                    string.push('"');
                }
                Some('\\') => {
                    len += 1;
                    string.push('\\');
                }
                Some('/') => {
                    len += 1;
                    string.push('/');
                }
                Some('b') => {
                    len += 1;
                    string.push('\x08');
                }
                Some('f') => {
                    len += 1;
                    string.push('\x0C');
                }
                Some('n') => {
                    len += 1;
                    string.push('\n');
                }
                Some('r') => {
                    len += 1;
                    string.push('\r');
                }
                Some('t') => {
                    len += 1;
                    string.push('\t');
                }
                Some('u') => std::todo!(),
                _ => return (SyntaxKind::ERROR, line.len() + 1, None),
            },
            _ => string.push(c),
        }
    }

    (SyntaxKind::STRING_VALUE, len, Some(string))
}

fn number(negate: bool, source: &str) -> (SyntaxKind, usize) {
    let (negate, rest) = if negate {
        (1, &source[1..])
    } else {
        (0, source)
    };
    let (integer_part, rest) = if rest.starts_with('0') {
        (1, &rest[1..])
    } else {
        let len = rest.chars().take_while(|c| c.is_ascii_digit()).count();
        if len == 0 {
            return (SyntaxKind::ERROR, negate);
        }

        (len, &rest[len..])
    };
    let (fraction_part, rest) = if rest.starts_with('.') {
        let len = rest
            .chars()
            .skip(1)
            .take_while(|c| c.is_ascii_digit())
            .count();
        if len == 0 {
            let len = integer_part + negate;

            return (SyntaxKind::INT_VALUE, len);
        }

        (len + 1, &rest[len + 1..])
    } else {
        (0, rest)
    };
    let exponent_part = if rest.starts_with(|c| matches!(c, 'e' | 'E')) {
        let rest = &rest[1..];
        let (sign_part, rest) = if rest.starts_with(|c| matches!(c, '+' | '-')) {
            (1, &rest[1..])
        } else {
            (0, rest)
        };
        let len = rest.chars().take_while(|c| c.is_ascii_digit()).count();
        if len == 0 {
            let len = integer_part + fraction_part + negate;

            return (SyntaxKind::INT_VALUE, len);
        }

        sign_part + len + 1
    } else {
        0
    };

    if fraction_part == 0 && exponent_part == 0 {
        let len = integer_part + negate;

        (SyntaxKind::INT_VALUE, len)
    } else {
        let len = integer_part + fraction_part + exponent_part + negate;

        (SyntaxKind::FLOAT_VALUE, len)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn punctuators() {
        let tokens = super::lex(r#"():=@$&[]{}!|..."#);
        let tokens: Vec<_> = tokens.collect();
        insta::assert_debug_snapshot!(tokens);
    }

    #[test]
    fn names() {
        let tokens = super::lex(r#"_ _123 foo foo123 true false null"#);
        let tokens: Vec<_> = tokens.collect();
        insta::assert_debug_snapshot!(tokens);
    }

    #[test]
    fn integers() {
        let tokens = super::lex(r#"0 123 -0 -123"#);
        let tokens: Vec<_> = tokens.collect();
        insta::assert_debug_snapshot!(tokens);
    }

    #[test]
    fn no_integers() {
        let tokens = super::lex(r#"0x 0. 0e 123_456 -123_456"#);
        let tokens: Vec<_> = tokens.collect();
        insta::assert_debug_snapshot!(tokens);
    }

    #[test]
    fn floats_with_fraction() {
        let tokens = super::lex(r#"0.0 123.0 123.456 -0.0 -123.0 -123.456"#);
        let tokens: Vec<_> = tokens.collect();
        insta::assert_debug_snapshot!(tokens);
    }

    #[test]
    fn floats_with_exponent() {
        let tokens = super::lex(r#"0e0 123e0 123e456 -0e0 -123e0 -123e456"#);
        let tokens: Vec<_> = tokens.collect();
        insta::assert_debug_snapshot!(tokens);
    }

    #[test]
    fn floats_with_fraction_and_exponent() {
        let tokens = super::lex(
            r#"0.0e0 123.0e0 123.456e0 123.0e456 123.456e789 -0.0e0 -123.0e0 -123.456e0 -123.0e456 -123.456e789"#,
        );
        let tokens: Vec<_> = tokens.collect();
        insta::assert_debug_snapshot!(tokens);
    }

    #[test]
    fn single_line_string() {
        let tokens = super::lex(r#""" "foo" "foo\"bar" "foo\nbar" "foo\rbar" "foo\r\nbar""#);
        let tokens: Vec<_> = tokens.collect();
        insta::assert_debug_snapshot!(tokens);
    }

    #[ignore]
    #[test]
    fn single_line_string_with_escape_unicode() {
        let tokens = super::lex(r#""\u1234""#);
        let tokens: Vec<_> = tokens.collect();
        insta::assert_debug_snapshot!(tokens);
    }

    #[ignore]
    #[test]
    fn block_string() {
        let tokens = super::lex(r#"""""" """"""""#);
        let tokens: Vec<_> = tokens.collect();
        insta::assert_debug_snapshot!(tokens);
    }

    #[test]
    fn comments() {
        let tokens = super::lex(r#"# foo"#);
        let tokens: Vec<_> = tokens.collect();
        insta::assert_debug_snapshot!(tokens);
    }

    #[test]
    fn ignored_tokens() {
        let tokens = super::lex(" ,\t\n\r");
        let tokens: Vec<_> = tokens.collect();
        insta::assert_debug_snapshot!(tokens);
    }
}
