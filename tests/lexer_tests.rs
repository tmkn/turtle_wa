#[cfg(test)]
use pretty_assertions::assert_eq;

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

#[test]
fn parse_object_list_iri() {
    let input = "<http://example.org/subject> <http://example.org/predicate> <http://example.org/object1>, <http://example.org/object2> .";

    let tokens = tokenize(&input, 0);

    assert_eq!(
        tokens,
        vec![
            Lexeme::Iri("http://example.org/subject".to_string()),
            Lexeme::Iri("http://example.org/predicate".to_string()),
            Lexeme::Iri("http://example.org/object1".to_string()),
            Lexeme::ObjectListToken,
            Lexeme::Iri("http://example.org/object2".to_string()),
            Lexeme::EndToken,
        ],
    );
}

#[test]
fn parse_object_list_literal() {
    let input = "<http://example.org/#spiderman> <http://xmlns.com/foaf/0.1/name> \"Spiderman\", \"Человек-паук\"@ru .";

    let tokens = tokenize(&input, 0);

    assert_eq!(
        tokens,
        vec![
            Lexeme::Iri("http://example.org/#spiderman".to_string()),
            Lexeme::Iri("http://xmlns.com/foaf/0.1/name".to_string()),
            Lexeme::Literal("Spiderman".to_string()),
            Lexeme::ObjectListToken,
            Lexeme::LangLiteral("Человек-паук".to_string(), "ru".to_string()),
            Lexeme::EndToken,
        ],
    );
}

#[test]
fn parse_object_list_literal_2() {
    let input = "<http://example.org/#spiderman> <http://xmlns.com/foaf/0.1/name> \"Человек-паук\"@ru, \"Spiderman\" .";

    let tokens = tokenize(&input, 0);

    assert_eq!(
        tokens,
        vec![
            Lexeme::Iri("http://example.org/#spiderman".to_string()),
            Lexeme::Iri("http://xmlns.com/foaf/0.1/name".to_string()),
            Lexeme::LangLiteral("Человек-паук".to_string(), "ru".to_string()),
            Lexeme::ObjectListToken,
            Lexeme::Literal("Spiderman".to_string()),
            Lexeme::EndToken,
        ],
    );
}

#[test]
fn parse_object_list_literal_mix() {
    let input = "<http://example.org/#spiderman> <http://xmlns.com/foaf/0.1/name> \"Человек-паук\"@ru, <http://example.com/object>, \"Spiderman\"^^<http://www.w3.org/2001/XMLSchema#string> .";

    let tokens = tokenize(&input, 0);

    assert_eq!(
        tokens,
        vec![
            Lexeme::Iri("http://example.org/#spiderman".to_string()),
            Lexeme::Iri("http://xmlns.com/foaf/0.1/name".to_string()),
            Lexeme::LangLiteral("Человек-паук".to_string(), "ru".to_string()),
            Lexeme::ObjectListToken,
            Lexeme::Iri("http://example.com/object".to_string()),
            Lexeme::ObjectListToken,
            Lexeme::DataTypeLiteral(
                "Spiderman".to_string(),
                "http://www.w3.org/2001/XMLSchema#string".to_string()
            ),
            Lexeme::EndToken,
        ],
    );
}
