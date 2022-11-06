use std::collections::HashMap;

use turtle_wa::lexer::*;
use turtle_wa::parser::*;

mod parser {
    mod parse_triple {
        use super::super::*;

        #[test]
        fn parse_simple_triple() {
            let lexemes: &Vec<Lexeme> = &vec![
                Lexeme::Iri("http://example.org/subject".to_string()),
                Lexeme::Iri("http://example.org/predicate".to_string()),
                Lexeme::Iri("http://example.org/object".to_string()),
                Lexeme::EndToken,
            ];
            let mut context = ParseContext::new();
            let triples = parse(&lexemes, &mut context);

            assert_eq!(
                triples,
                vec![Triple {
                    subject: Iri("http://example.org/subject".to_string()),
                    predicate: Iri("http://example.org/predicate".to_string()),
                    object: Object::Iri("http://example.org/object".to_string()),
                }]
            );
        }

        #[test]
        fn parse_triple_with_a_predicate() {
            let lexemes: &Vec<Lexeme> = &vec![
                Lexeme::Iri("http://example.org/subject".to_string()),
                Lexeme::A,
                Lexeme::Iri("http://example.org/object".to_string()),
                Lexeme::EndToken,
            ];
            let mut context = ParseContext::new();
            let triples = parse(&lexemes, &mut context);

            assert_eq!(
                triples,
                vec![Triple {
                    subject: Iri("http://example.org/subject".to_string()),
                    predicate: Iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#type".to_string()),
                    object: Object::Iri("http://example.org/object".to_string()),
                }]
            );
        }
    }

    mod relative_and_absolute_uri {
        mod no_base_and_no_prefixes {
            use super::super::super::*;

            #[test]
            fn parse_absolute_iri() {
                let context = ParseContext::new();

                let result = parse_iri(Lexeme::Iri("http://example.org/foo".to_string()), context);

                assert_eq!(result, Some(Iri("http://example.org/foo".to_string())));
            }

            #[test]
            fn parse_relative_iri() {
                let context = ParseContext {
                    base: Some(String::from("http://example.org/")),
                    prefixes: HashMap::new(),
                };

                let result = parse_iri(Lexeme::Iri("foo".to_string()), context);

                assert_eq!(result, Some(Iri("http://example.org/foo".to_string())));
            }
        }
    }

    mod prefixed_uri {
        use super::super::*;

        #[test]
        fn parse_prefixed_uri() {
            let context = ParseContext {
                base: None,
                prefixes: HashMap::from([(
                    String::from("foo:"),
                    String::from("http://example.org/"),
                )]),
            };

            let result = parse_iri(Lexeme::PrefixedIri("foo:bar".to_string()), context);

            assert_eq!(result, Some(Iri("http://example.org/bar".to_string())));
        }

        #[test]
        fn parse_prefixed_uri_simple_prefix() {
            let context = ParseContext {
                base: None,
                prefixes: HashMap::from([(String::from(":"), String::from("http://example.org/"))]),
            };

            let result = parse_iri(Lexeme::PrefixedIri(":bar".to_string()), context);

            assert_eq!(result, Some(Iri("http://example.org/bar".to_string())));
        }

        #[test]
        fn parse_prefixed_uri_with_no_prefix_set() {
            let context = ParseContext {
                base: None,
                prefixes: HashMap::from([]),
            };

            let result = parse_iri(Lexeme::PrefixedIri("foo:bar".to_string()), context);

            assert_eq!(result, None);
        }
    }
}
