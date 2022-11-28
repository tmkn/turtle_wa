// #[cfg(test)]
// use pretty_assertions::assert_eq;

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

        #[test]
        fn parse_object_list_iri() {
            let lexemes: &Vec<Lexeme> = &vec![
                Lexeme::Iri("http://example.org/subject".to_string()),
                Lexeme::Iri("http://example.org/predicate".to_string()),
                Lexeme::Iri("http://example.org/object1".to_string()),
                Lexeme::ObjectListToken,
                Lexeme::Iri("http://example.org/object2".to_string()),
                Lexeme::EndToken,
            ];
            let mut context = ParseContext::new();
            let triples = parse(&lexemes, &mut context);

            assert_eq!(
                triples,
                vec![
                    Triple {
                        subject: Iri("http://example.org/subject".to_string()),
                        predicate: Iri("http://example.org/predicate".to_string()),
                        object: Object::Iri("http://example.org/object1".to_string()),
                    },
                    Triple {
                        subject: Iri("http://example.org/subject".to_string()),
                        predicate: Iri("http://example.org/predicate".to_string()),
                        object: Object::Iri("http://example.org/object2".to_string()),
                    },
                ]
            );
        }

        #[test]
        fn parse_object_list_literal() {
            let lexemes: &Vec<Lexeme> = &vec![
                Lexeme::Iri("http://example.org/subject".to_string()),
                Lexeme::Iri("http://example.org/predicate".to_string()),
                Lexeme::LangLiteral("Spiderman".to_string(), "en".to_string()),
                Lexeme::ObjectListToken,
                Lexeme::LangLiteral("Человек-паук".to_string(), "ru".to_string()),
                Lexeme::EndToken,
            ];
            let mut context = ParseContext::new();
            let triples = parse(&lexemes, &mut context);

            assert_eq!(
                triples,
                vec![
                    Triple {
                        subject: Iri("http://example.org/subject".to_string()),
                        predicate: Iri("http://example.org/predicate".to_string()),
                        object: Object::LangLiteral("Spiderman".to_string(), "en".to_string()),
                    },
                    Triple {
                        subject: Iri("http://example.org/subject".to_string()),
                        predicate: Iri("http://example.org/predicate".to_string()),
                        object: Object::LangLiteral("Человек-паук".to_string(), "ru".to_string()),
                    },
                ]
            );
        }
    }

    mod relative_and_absolute_uri {
        mod no_base_and_no_prefixes {
            use super::super::super::*;

            #[test]
            fn parse_absolute_iri() {
                let context = ParseContext::new();

                let result =
                    parse_iri(&Lexeme::Iri("http://example.org/foo".to_string()), &context);

                assert_eq!(result, Some(Iri("http://example.org/foo".to_string())));
            }

            #[test]
            fn parse_relative_iri() {
                let context = ParseContext {
                    base: Some(String::from("http://example.org/")),
                    prefixes: HashMap::new(),
                    subject: None,
                };

                let result = parse_iri(&Lexeme::Iri("foo".to_string()), &context);

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
                subject: None,
            };

            let result = parse_iri(&Lexeme::PrefixedIri("foo:bar".to_string()), &context);

            assert_eq!(result, Some(Iri("http://example.org/bar".to_string())));
        }

        #[test]
        fn parse_multiple_prefixed_uri() {
            let lexemes = vec![
                Lexeme::Prefix(":".to_string(), "http://example.org/".to_string()),
                Lexeme::EndToken,
                Lexeme::PrefixedIri(":subject".to_string()),
                Lexeme::PrefixedIri(":predicate".to_string()),
                Lexeme::PrefixedIri(":object".to_string()),
                Lexeme::EndToken,
                Lexeme::Prefix(
                    "foaf:".to_string(),
                    "http://xmlns.com/foaf/0.1/".to_string(),
                ),
                Lexeme::EndToken,
                Lexeme::PrefixedIri(":subject".to_string()),
                Lexeme::PrefixedIri("foaf:name".to_string()),
                Lexeme::Literal("Alice".to_string()),
                Lexeme::EndToken,
            ];
            let mut context = ParseContext::new();

            let triples = parse(&lexemes, &mut context);

            assert_eq!(
                triples,
                vec![
                    Triple {
                        subject: Iri("http://example.org/subject".to_string()),
                        predicate: Iri("http://example.org/predicate".to_string()),
                        object: Object::Iri("http://example.org/object".to_string()),
                    },
                    Triple {
                        subject: Iri("http://example.org/subject".to_string()),
                        predicate: Iri("http://xmlns.com/foaf/0.1/name".to_string()),
                        object: Object::Literal("Alice".to_string()),
                    },
                ]
            );
        }

        #[test]
        fn parse_prefixed_uri_simple_prefix() {
            let context = ParseContext {
                base: None,
                prefixes: HashMap::from([(String::from(":"), String::from("http://example.org/"))]),
                subject: None,
            };

            let result = parse_iri(&Lexeme::PrefixedIri(":bar".to_string()), &context);

            assert_eq!(result, Some(Iri("http://example.org/bar".to_string())));
        }

        #[test]
        fn parse_prefixed_uri_with_no_prefix_set() {
            let context = ParseContext::new();

            let result = parse_iri(&Lexeme::PrefixedIri("foo:bar".to_string()), &context);

            assert_eq!(result, None);
        }
    }

    mod base {
        use super::super::*;

        #[test]
        fn parse_single_base() {
            let lexemes: &Vec<Lexeme> = &vec![
                Lexeme::Base("http://example.org/".to_string()),
                Lexeme::Iri("subject1".to_string()),
                Lexeme::Iri("predicate1".to_string()),
                Lexeme::Iri("object1".to_string()),
                Lexeme::EndToken,
            ];
            let mut context = ParseContext::new();
            let triples = parse(&lexemes, &mut context);

            assert_eq!(context.base, Some("http://example.org/".to_string()));
            assert_eq!(
                triples,
                vec![Triple {
                    subject: Iri("http://example.org/subject1".to_string()),
                    predicate: Iri("http://example.org/predicate1".to_string()),
                    object: Object::Iri("http://example.org/object1".to_string()),
                },]
            );
        }

        #[test]
        fn parse_multi_base() {
            let lexemes: &Vec<Lexeme> = &vec![
                Lexeme::Base("http://example1.org/".to_string()),
                Lexeme::Iri("subject1".to_string()),
                Lexeme::Iri("predicate1".to_string()),
                Lexeme::Iri("object1".to_string()),
                Lexeme::EndToken,
                Lexeme::Base("http://example2.com/".to_string()),
                Lexeme::Iri("subject2".to_string()),
                Lexeme::Iri("predicate2".to_string()),
                Lexeme::Iri("object2".to_string()),
                Lexeme::EndToken,
            ];
            let mut context = ParseContext::new();
            let triples = parse(&lexemes, &mut context);

            assert_eq!(context.base, Some("http://example2.com/".to_string()));
            assert_eq!(
                triples,
                vec![
                    Triple {
                        subject: Iri("http://example1.org/subject1".to_string()),
                        predicate: Iri("http://example1.org/predicate1".to_string()),
                        object: Object::Iri("http://example1.org/object1".to_string()),
                    },
                    Triple {
                        subject: Iri("http://example2.com/subject2".to_string()),
                        predicate: Iri("http://example2.com/predicate2".to_string()),
                        object: Object::Iri("http://example2.com/object2".to_string()),
                    },
                ]
            );
        }
    }

    mod boolean {
        use super::super::*;

        #[test]
        fn parse_false() {
            let lexemes: &Vec<Lexeme> = &vec![
                Lexeme::Prefix(":".to_string(), "http://example.org/stats".to_string()),
                Lexeme::EndToken,
                Lexeme::Iri("http://somecountry.example/census2007".to_string()),
                Lexeme::PrefixedIri(":isLandlocked".to_string()),
                Lexeme::Unknown("false".to_string()),
                Lexeme::EndToken,
                Lexeme::Comment(" xsd:boolean".to_string()),
            ];
            let mut context = ParseContext::new();
            let triples = parse(&lexemes, &mut context);

            assert_eq!(
                triples,
                vec![Triple {
                    subject: Iri("http://somecountry.example/census2007".to_string()),
                    predicate: Iri("http://example.org/statsisLandlocked".to_string()),
                    object: Object::Boolean(false),
                },]
            );
        }

        #[test]
        fn parse_true() {
            let lexemes: &Vec<Lexeme> = &vec![
                Lexeme::Prefix(":".to_string(), "http://example.org/stats".to_string()),
                Lexeme::EndToken,
                Lexeme::Iri("http://somecountry.example/census2007".to_string()),
                Lexeme::PrefixedIri(":isLandlocked".to_string()),
                Lexeme::Unknown("true".to_string()),
                Lexeme::EndToken,
                Lexeme::Comment(" xsd:boolean".to_string()),
            ];
            let mut context = ParseContext::new();
            let triples = parse(&lexemes, &mut context);

            assert_eq!(
                triples,
                vec![Triple {
                    subject: Iri("http://somecountry.example/census2007".to_string()),
                    predicate: Iri("http://example.org/statsisLandlocked".to_string()),
                    object: Object::Boolean(true),
                },]
            );
        }
    }

    mod numbers {
        mod integers {
            use super::super::super::*;

            #[test]
            fn parse_positive_integer() {
                let lexemes: &Vec<Lexeme> = &vec![
                    Lexeme::Iri("http://somecountry.example/census2007".to_string()),
                    Lexeme::Iri("http://example.org/stats/population".to_string()),
                    Lexeme::Unknown("1234567890".to_string()),
                    Lexeme::EndToken,
                ];
                let mut context = ParseContext::new();
                let triples = parse(&lexemes, &mut context);

                assert_eq!(
                    triples,
                    vec![Triple {
                        subject: Iri("http://somecountry.example/census2007".to_string()),
                        predicate: Iri("http://example.org/stats/population".to_string()),
                        object: Object::Integer(1234567890),
                    },]
                );
            }

            #[test]
            fn parse_negative_integer() {
                let lexemes: &Vec<Lexeme> = &vec![
                    Lexeme::Iri("http://somecountry.example/census2007".to_string()),
                    Lexeme::Iri("http://example.org/stats/population".to_string()),
                    Lexeme::Unknown("-1234567890".to_string()),
                    Lexeme::EndToken,
                ];
                let mut context = ParseContext::new();
                let triples = parse(&lexemes, &mut context);

                assert_eq!(
                    triples,
                    vec![Triple {
                        subject: Iri("http://somecountry.example/census2007".to_string()),
                        predicate: Iri("http://example.org/stats/population".to_string()),
                        object: Object::Integer(-1234567890),
                    },]
                );
            }
        }

        mod decimals {
            use super::super::super::*;

            #[test]
            fn parse_positive_integer() {
                let lexemes: &Vec<Lexeme> = &vec![
                    Lexeme::Iri("http://somecountry.example/census2007".to_string()),
                    Lexeme::Iri("http://example.org/stats/area".to_string()),
                    Lexeme::Unknown("4.002602".to_string()),
                    Lexeme::EndToken,
                ];
                let mut context = ParseContext::new();
                let triples = parse(&lexemes, &mut context);

                assert_eq!(
                    triples,
                    vec![Triple {
                        subject: Iri("http://somecountry.example/census2007".to_string()),
                        predicate: Iri("http://example.org/stats/area".to_string()),
                        object: Object::Decimal(4.002602),
                    },]
                );
            }

            #[test]
            fn parse_negative_decimal() {
                let lexemes: &Vec<Lexeme> = &vec![
                    Lexeme::Iri("http://somecountry.example/census2007".to_string()),
                    Lexeme::Iri("http://example.org/stats/area".to_string()),
                    Lexeme::Unknown("-4.002602".to_string()),
                    Lexeme::EndToken,
                ];
                let mut context = ParseContext::new();
                let triples = parse(&lexemes, &mut context);

                assert_eq!(
                    triples,
                    vec![Triple {
                        subject: Iri("http://somecountry.example/census2007".to_string()),
                        predicate: Iri("http://example.org/stats/area".to_string()),
                        object: Object::Decimal(-4.002602),
                    },]
                );
            }
        }

        mod doubles {
            use super::super::super::*;

            #[test]
            fn parse_positive_double() {
                let lexemes: &Vec<Lexeme> = &vec![
                    Lexeme::Iri("http://somecountry.example/census2007".to_string()),
                    Lexeme::Iri("http://example.org/stats/area".to_string()),
                    Lexeme::Unknown("1.663E-4".to_string()),
                    Lexeme::EndToken,
                ];
                let mut context = ParseContext::new();
                let triples = parse(&lexemes, &mut context);

                assert_eq!(
                    triples,
                    vec![Triple {
                        subject: Iri("http://somecountry.example/census2007".to_string()),
                        predicate: Iri("http://example.org/stats/area".to_string()),
                        object: Object::Double(0.0001663),
                    },]
                );
            }

            #[test]
            fn parse_negative_double() {
                let lexemes: &Vec<Lexeme> = &vec![
                    Lexeme::Iri("http://somecountry.example/census2007".to_string()),
                    Lexeme::Iri("http://example.org/stats/area".to_string()),
                    Lexeme::Unknown("-1.663E-4".to_string()),
                    Lexeme::EndToken,
                ];
                let mut context = ParseContext::new();
                let triples = parse(&lexemes, &mut context);

                assert_eq!(
                    triples,
                    vec![Triple {
                        subject: Iri("http://somecountry.example/census2007".to_string()),
                        predicate: Iri("http://example.org/stats/area".to_string()),
                        object: Object::Double(-0.0001663),
                    },]
                );
            }
        }
    }

    mod multiline {
        use super::super::*;

        #[test]
        fn parse_multiline() {
            let lexemes: &Vec<Lexeme> = &vec![
                Lexeme::Iri("http://somecountry.example/census2007".to_string()),
                Lexeme::Iri("http://example.org/stats/gravity".to_string()),
                Lexeme::MultilineLiteral("hello multi\n\n line \"\" literal".to_string()),
                Lexeme::EndToken,
                Lexeme::Iri("http://somecountry.example/census2007".to_string()),
                Lexeme::Iri("http://example.org/stats/gravity".to_string()),
                Lexeme::MultilineLiteral("hello multi\n\n line 2\"\" literal".to_string()),
                Lexeme::EndToken,
            ];
            let mut context = ParseContext::new();
            let triples = parse(&lexemes, &mut context);

            assert_eq!(
                triples,
                vec![
                    Triple {
                        subject: Iri("http://somecountry.example/census2007".to_string()),
                        predicate: Iri("http://example.org/stats/gravity".to_string()),
                        object: Object::MultilineLiteral(
                            "hello multi\n\n line \"\" literal".to_string()
                        ),
                    },
                    Triple {
                        subject: Iri("http://somecountry.example/census2007".to_string()),
                        predicate: Iri("http://example.org/stats/gravity".to_string()),
                        object: Object::MultilineLiteral(
                            "hello multi\n\n line 2\"\" literal".to_string()
                        ),
                    },
                ]
            );
        }
    }

    mod parse_literal {
        use super::super::*;

        #[test]
        fn parse_string_literal() {
            let lexemes: &Vec<Lexeme> = &vec![
                Lexeme::Iri("http://somecountry.example/census2007".to_string()),
                Lexeme::Iri("http://example.org/stats/area".to_string()),
                Lexeme::Literal("This is a string literal".to_string()),
                Lexeme::EndToken,
            ];
            let mut context = ParseContext::new();
            let triples = parse(&lexemes, &mut context);

            assert_eq!(
                triples,
                vec![Triple {
                    subject: Iri("http://somecountry.example/census2007".to_string()),
                    predicate: Iri("http://example.org/stats/area".to_string()),
                    object: Object::Literal("This is a string literal".to_string()),
                },]
            );
        }

        #[test]
        fn parse_string_literal_with_language() {
            let lexemes: &Vec<Lexeme> = &vec![
                Lexeme::Iri("http://somecountry.example/census2007".to_string()),
                Lexeme::Iri("http://example.org/stats/area".to_string()),
                Lexeme::LangLiteral("This is a string literal".to_string(), "en".to_string()),
                Lexeme::EndToken,
            ];
            let mut context = ParseContext::new();
            let triples = parse(&lexemes, &mut context);

            assert_eq!(
                triples,
                vec![Triple {
                    subject: Iri("http://somecountry.example/census2007".to_string()),
                    predicate: Iri("http://example.org/stats/area".to_string()),
                    object: Object::LangLiteral(
                        "This is a string literal".to_string(),
                        "en".to_string()
                    ),
                },]
            );
        }

        #[test]
        fn parse_with_datatype() {
            let lexemes = &vec![
                Lexeme::Iri("http://example.org/vocab/show/218".to_string()),
                Lexeme::Iri("http://www.w3.org/2000/01/rdf-schema#label".to_string()),
                Lexeme::DataTypeLiteral(
                    "That Seventies Show".to_string(),
                    "http://www.w3.org/2001/XMLSchema#string".to_string(),
                ),
                Lexeme::EndToken,
            ];
            let mut context = ParseContext::new();

            let triples = parse(&lexemes, &mut context);

            assert_eq!(
                triples,
                vec![Triple {
                    subject: Iri("http://example.org/vocab/show/218".to_string()),
                    predicate: Iri("http://www.w3.org/2000/01/rdf-schema#label".to_string()),
                    object: Object::DataTypeLiteral(
                        "That Seventies Show".to_string(),
                        "http://www.w3.org/2001/XMLSchema#string".to_string()
                    ),
                },]
            );
        }

        #[test]
        fn parse_with_datatype_prefixed_iri() {
            let lexemes = &vec![
                Lexeme::Prefix(
                    "xsd:".to_string(),
                    "http://www.w3.org/2001/XMLSchema#".to_string(),
                ),
                Lexeme::EndToken,
                Lexeme::Iri("http://example.org/vocab/show/218".to_string()),
                Lexeme::Iri("http://www.w3.org/2000/01/rdf-schema#label".to_string()),
                Lexeme::DataTypeLiteral(
                    "That Seventies Show".to_string(),
                    "xsd:string".to_string(),
                ),
                Lexeme::EndToken,
            ];
            let mut context = ParseContext::new();

            let triples = parse(&lexemes, &mut context);

            assert_eq!(
                triples,
                vec![Triple {
                    subject: Iri("http://example.org/vocab/show/218".to_string()),
                    predicate: Iri("http://www.w3.org/2000/01/rdf-schema#label".to_string()),
                    object: Object::DataTypeLiteral(
                        "That Seventies Show".to_string(),
                        "http://www.w3.org/2001/XMLSchema#string".to_string()
                    ),
                },]
            );
        }
    }
}
