use http::Uri;
use std::collections::HashMap;

// todo parser
use crate::lexer::*;

#[derive(PartialEq, Debug)]
pub enum Object {
    Iri(String),
    Literal(String),
    LangLiteral(String, String),
    DataTypeLiteral(String, String),
}

impl TryFrom<Lexeme> for Object {
    type Error = ();

    fn try_from(lexeme: Lexeme) -> Result<Self, Self::Error> {
        match lexeme {
            Lexeme::Iri(iri) => Ok(Object::Iri(iri)),
            Lexeme::Literal(literal) => Ok(Object::Literal(literal)),
            Lexeme::LangLiteral(literal, lang) => Ok(Object::LangLiteral(literal, lang)),
            Lexeme::DataTypeLiteral(literal, datatype) => {
                Ok(Object::DataTypeLiteral(literal, datatype))
            }
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Iri(pub String);

impl TryFrom<Lexeme> for Iri {
    type Error = ();

    fn try_from(lexeme: Lexeme) -> Result<Self, Self::Error> {
        match lexeme {
            Lexeme::Iri(iri) => Ok(Iri(iri)),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Triple {
    pub subject: Iri,
    pub predicate: Iri,
    pub object: Object,
}

pub struct ParseContext {
    pub base: Option<String>,
    pub prefixes: HashMap<String, String>,
}

impl ParseContext {
    pub fn new() -> ParseContext {
        ParseContext {
            base: None,
            prefixes: HashMap::new(),
        }
    }
}

pub fn parse(lexemes: &Vec<Lexeme>, context: &mut ParseContext) -> Vec<Triple> {
    // todo
    let mut triples: Vec<Triple> = Vec::new();
    let mut current_triple: (Option<Iri>, Option<Iri>, Option<Object>) = (None, None, None);
    let mut itr = lexemes.iter().peekable();
    //let mut context = ParseContext::new();

    while let Some(lexeme) = itr.next() {
        match lexeme {
            Lexeme::Iri(iri) => match current_triple {
                (None, None, None) => {
                    current_triple.0 = Some(Iri(iri.to_string()));
                }
                (Some(_), None, None) => {
                    current_triple.1 = Some(Iri(iri.to_string()));
                }
                (Some(_), Some(_), None) => {
                    current_triple.2 = Some(Object::Iri(iri.to_string()));
                }
                _ => {}
            },
            Lexeme::PrefixedIri(prefixed_iri) => {
                let iri =
                    parse_prefixed_iri(Lexeme::PrefixedIri(prefixed_iri.to_string()), &context);

                match iri {
                    Some(iri) => match current_triple {
                        (None, None, None) => {
                            current_triple.0 = Some(iri);
                        }
                        (Some(_), None, None) => {
                            current_triple.1 = Some(iri);
                        }
                        (Some(_), Some(_), None) => {
                            current_triple.2 = Some(Object::Iri(iri.0));
                        }
                        _ => {}
                    },
                    _ => {
                        println!("Error parsing prefixed iri: {}", prefixed_iri)
                    }
                }
            }
            Lexeme::Literal(literal) => {
                current_triple.2 = Some(Object::Literal(literal.to_string()));
            }
            Lexeme::LangLiteral(literal, lang) => {
                current_triple.2 = Some(Object::LangLiteral(literal.to_string(), lang.to_string()));
            }
            Lexeme::DataTypeLiteral(literal, datatype) => {
                current_triple.2 = Some(Object::DataTypeLiteral(
                    literal.to_string(),
                    datatype.to_string(),
                ));
            }
            Lexeme::EndToken => {
                match current_triple {
                    (Some(subject), Some(predicate), Some(object)) => {
                        triples.push(Triple {
                            subject,
                            predicate,
                            object,
                        });
                    }
                    _ => {}
                }

                current_triple = (None, None, None);
            }
            Lexeme::Prefix(key, value) => {
                context.prefixes.insert(key.to_string(), value.to_string());
            }
            Lexeme::A => {
                if let (Some(_), None, None) = current_triple {
                    current_triple.1 = Some(Iri(
                        "http://www.w3.org/1999/02/22-rdf-syntax-ns#type".to_string()
                    ));
                }
            }
            _ => {
                // ignore other lexemes for now
            }
        }
    }

    triples
}

pub fn parse_iri(lexeme: Lexeme, context: ParseContext) -> Option<Iri> {
    match lexeme {
        Lexeme::Iri(iri) => match is_relative_iri(&iri.to_string()) {
            true => match context.base {
                Some(base) => {
                    let mut full_iri = base.clone();

                    full_iri.push_str(&iri);

                    Some(Iri(full_iri))
                }
                None => None,
            },
            false => Some(Iri(iri)),
        },
        Lexeme::PrefixedIri(prefixed_iri) => {
            parse_prefixed_iri(Lexeme::PrefixedIri(prefixed_iri), &context)
        }
        _ => None,
    }
}

fn is_relative_iri(iri: &str) -> bool {
    let parsed_iri = iri.parse::<Uri>().unwrap();

    match parsed_iri.scheme() {
        Some(_) => false,
        None => true,
    }
}

fn parse_prefixed_iri(lexeme: Lexeme, context: &ParseContext) -> Option<Iri> {
    match lexeme {
        Lexeme::PrefixedIri(ref prefixed_iri) => {
            let parts = prefixed_iri.split(':').collect::<Vec<&str>>();

            let (prefix, relative_iri) = (parts.first(), parts.get(1));

            match (prefix, relative_iri) {
                (Some(prefix), Some(relative_iri)) => {
                    let iri_prefix = format!("{}:", prefix);

                    match context.prefixes.get(&iri_prefix) {
                        Some(prefix) => {
                            let mut prefix = prefix.to_string();

                            prefix.push_str(relative_iri);

                            Some(Iri(prefix))
                        }
                        None => None,
                    }
                }
                _ => None,
            }
        }
        _ => None,
    }
}
