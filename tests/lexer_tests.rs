use turtle_wa::lexer::*;

#[test]
fn parse_base_turtle() {
    let base = "@base <http://example.org/> .";
    let tokens = tokenize(base, 0);

    assert_eq!(
        tokens,
        vec![
            Lexeme::Base("http://example.org/".to_string()),
            Lexeme::EndToken,
        ],
    );
}

#[test]
fn parse_base_sparql() {
    let base = "BASE <http://example.org/> .";
    let tokens = tokenize(base, 0);

    assert_eq!(
        tokens,
        vec![
            Lexeme::Base("http://example.org/".to_string()),
            Lexeme::EndToken,
        ],
    );
}

#[test]
fn parse_prefix_turtle_only_colon() {
    let tokens = tokenize("@prefix : <http://example.org/> .", 0);

    assert_eq!(
        tokens,
        vec![
            Lexeme::Prefix(":".to_string(), "http://example.org/".to_string()),
            Lexeme::EndToken,
        ],
    );
}

#[test]
fn parse_prefix_sparql_only_colon() {
    let tokens = tokenize("PREFIX : <http://example.org/> .", 0);

    assert_eq!(
        tokens,
        vec![
            Lexeme::Prefix(":".to_string(), "http://example.org/".to_string()),
            Lexeme::EndToken,
        ],
    );
}

#[test]
fn parse_prefix_turtle() {
    let tokens = tokenize("@prefix foo: <http://example.org/> .", 0);

    assert_eq!(
        tokens,
        vec![
            Lexeme::Prefix("foo:".to_string(), "http://example.org/".to_string()),
            Lexeme::EndToken,
        ],
    );
}

#[test]
fn parse_prefix_sparql() {
    let tokens = tokenize("PREFIX foo: <http://example.org/> .", 0);

    assert_eq!(
        tokens,
        vec![
            Lexeme::Prefix("foo:".to_string(), "http://example.org/".to_string()),
            Lexeme::EndToken,
        ],
    );
}
