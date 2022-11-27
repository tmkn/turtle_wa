#[cfg(test)]
use pretty_assertions::assert_eq;

use turtle_wa::lexer::*;

#[test]
fn parse_base_turtle() {
    let base = "@base <http://example.org/> .";
    let tokens = tokenize(base, 0, &mut LexerContext::new());

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
    let tokens = tokenize(base, 0, &mut LexerContext::new());

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
    let tokens = tokenize(
        "@prefix : <http://example.org/> .",
        0,
        &mut LexerContext::new(),
    );

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
    let tokens = tokenize(
        "PREFIX : <http://example.org/> .",
        0,
        &mut LexerContext::new(),
    );

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
    let tokens = tokenize(
        "@prefix foo: <http://example.org/> .",
        0,
        &mut LexerContext::new(),
    );

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
    let tokens = tokenize(
        "PREFIX foo: <http://example.org/> .",
        0,
        &mut LexerContext::new(),
    );

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
    let mut lexer_context = LexerContext::new();

    let tokens = tokenize(&input, 0, &mut lexer_context);

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
fn parse_comment() {
    let input = "<http://one.example/subject1> <http://one.example/predicate1> <http://one.example/object1> . # A triple with all absolute IRIs";
    let mut lexer_context = LexerContext::new();

    let tokens = tokenize(&input, 0, &mut lexer_context);

    assert_eq!(
        tokens,
        vec![
            Lexeme::Iri("http://one.example/subject1".to_string()),
            Lexeme::Iri("http://one.example/predicate1".to_string()),
            Lexeme::Iri("http://one.example/object1".to_string()),
            Lexeme::EndToken,
            Lexeme::Comment(" A triple with all absolute IRIs".to_string()),
        ],
    );
}

#[test]
fn parse_object_list_literal() {
    let input = "<http://example.org/#spiderman> <http://xmlns.com/foaf/0.1/name> \"Spiderman\", \"Человек-паук\"@ru .";
    let mut lexer_context = LexerContext::new();

    let tokens = tokenize(&input, 0, &mut lexer_context);

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
    let mut lexer_context = LexerContext::new();

    let tokens = tokenize(&input, 0, &mut lexer_context);

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
    let mut lexer_context = LexerContext::new();

    let tokens = tokenize(&input, 0, &mut lexer_context);

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

#[test]
fn parse_prefixed_uris() {
    let input = vec![
        "@prefix : <http://example.org/> .",
        ":subject :predicate :object .",
        "@prefix foaf: <http://xmlns.com/foaf/0.1/> .",
        ":subject foaf:name \"Alice\" .",
    ]
    .join("\n");
    let mut lexer_context = LexerContext::new();

    let tokens = tokenize(&input, 0, &mut lexer_context);

    assert_eq!(
        tokens,
        vec![
            Lexeme::Prefix(":".to_string(), "http://example.org/".to_string()),
            Lexeme::EndToken,
            Lexeme::PrefixedIri(":subject".to_string()),
            Lexeme::PrefixedIri(":predicate".to_string()),
            Lexeme::PrefixedIri(":object".to_string()),
            Lexeme::EndToken,
            Lexeme::Prefix(
                "foaf:".to_string(),
                "http://xmlns.com/foaf/0.1/".to_string()
            ),
            Lexeme::EndToken,
            Lexeme::PrefixedIri(":subject".to_string()),
            Lexeme::PrefixedIri("foaf:name".to_string()),
            Lexeme::Literal("Alice".to_string()),
            Lexeme::EndToken,
        ],
    );
}

#[test]
fn parse_predicate_list() {
    let input = vec![
        "<http://example.org/#spiderman> <http://www.perceive.net/schemas/relationship/enemyOf> <http://example.org/#green-goblin> ;",
        "        <http://xmlns.com/foaf/0.1/name> \"Spiderman\"@de ;",
        " 				<http://xmlns.com/foaf/0.1/name> \"Spiderman\" ."]
        .join("\n");
    let mut lexer_context = LexerContext::new();

    let tokens = tokenize(&input, 0, &mut lexer_context);

    assert_eq!(
        tokens,
        vec![
            Lexeme::Iri("http://example.org/#spiderman".to_string()),
            Lexeme::Iri("http://www.perceive.net/schemas/relationship/enemyOf".to_string()),
            Lexeme::Iri("http://example.org/#green-goblin".to_string()),
            Lexeme::PredicateListToken,
            Lexeme::Iri("http://xmlns.com/foaf/0.1/name".to_string()),
            Lexeme::LangLiteral("Spiderman".to_string(), "de".to_string()),
            Lexeme::PredicateListToken,
            Lexeme::Iri("http://xmlns.com/foaf/0.1/name".to_string()),
            Lexeme::Literal("Spiderman".to_string()),
            Lexeme::EndToken,
        ],
    );
}

#[test]
fn parse_boolean() {
    let input = vec![
        "@prefix : <http://example.org/stats> .",
        "<http://somecountry.example/census2007>",
        "    :isLandlocked false .           # xsd:boolean",
    ]
    .join("\n");
    let mut lexer_context = LexerContext::new();

    let tokens = tokenize(&input, 0, &mut lexer_context);

    assert_eq!(
        tokens,
        vec![
            Lexeme::Prefix(":".to_string(), "http://example.org/stats".to_string()),
            Lexeme::EndToken,
            Lexeme::Iri("http://somecountry.example/census2007".to_string()),
            Lexeme::PrefixedIri(":isLandlocked".to_string()),
            Lexeme::Unknown("false".to_string()),
            Lexeme::EndToken,
            Lexeme::Comment(" xsd:boolean".to_string()),
        ],
    );
}

#[test]
fn parse_integer() {
    let input =
        "<http://somecountry.example/census2007> <http://example.org/stats/population> 1234567890 .";
    let mut lexer_context = LexerContext::new();

    let tokens = tokenize(&input, 0, &mut lexer_context);

    assert_eq!(
        tokens,
        vec![
            Lexeme::Iri("http://somecountry.example/census2007".to_string()),
            Lexeme::Iri("http://example.org/stats/population".to_string()),
            Lexeme::Unknown("1234567890".to_string()),
            Lexeme::EndToken,
        ],
    );
}

#[test]
fn parse_negative_integer() {
    let input =
        "<http://somecountry.example/census2007> <http://example.org/stats/population> -1234567890 .";
    let mut lexer_context = LexerContext::new();

    let tokens = tokenize(&input, 0, &mut lexer_context);

    assert_eq!(
        tokens,
        vec![
            Lexeme::Iri("http://somecountry.example/census2007".to_string()),
            Lexeme::Iri("http://example.org/stats/population".to_string()),
            Lexeme::Unknown("-1234567890".to_string()),
            Lexeme::EndToken,
        ],
    );
}

#[test]
fn parse_decimal() {
    let input =
        "<http://somecountry.example/census2007> <http://example.org/stats/area> 4.002602 .";
    let mut lexer_context = LexerContext::new();

    let tokens = tokenize(&input, 0, &mut lexer_context);

    assert_eq!(
        tokens,
        vec![
            Lexeme::Iri("http://somecountry.example/census2007".to_string()),
            Lexeme::Iri("http://example.org/stats/area".to_string()),
            Lexeme::Unknown("4.002602".to_string()),
            Lexeme::EndToken,
        ],
    );
}

#[test]
fn parse_negative_decimal() {
    let input =
        "<http://somecountry.example/census2007> <http://example.org/stats/area> -4.002602 .";
    let mut lexer_context = LexerContext::new();

    let tokens = tokenize(&input, 0, &mut lexer_context);

    assert_eq!(
        tokens,
        vec![
            Lexeme::Iri("http://somecountry.example/census2007".to_string()),
            Lexeme::Iri("http://example.org/stats/area".to_string()),
            Lexeme::Unknown("-4.002602".to_string()),
            Lexeme::EndToken,
        ],
    );
}

#[test]
fn parse_double() {
    let input =
        "<http://somecountry.example/census2007> <http://example.org/stats/gravity> 1.663E-4 .";
    let mut lexer_context = LexerContext::new();

    let tokens = tokenize(&input, 0, &mut lexer_context);

    assert_eq!(
        tokens,
        vec![
            Lexeme::Iri("http://somecountry.example/census2007".to_string()),
            Lexeme::Iri("http://example.org/stats/gravity".to_string()),
            Lexeme::Unknown("1.663E-4".to_string()),
            Lexeme::EndToken,
        ],
    );
}

#[test]
fn parse_negative_double() {
    let input =
        "<http://somecountry.example/census2007> <http://example.org/stats/gravity> -1.663E-4 .";
    let mut lexer_context = LexerContext::new();

    let tokens = tokenize(&input, 0, &mut lexer_context);

    assert_eq!(
        tokens,
        vec![
            Lexeme::Iri("http://somecountry.example/census2007".to_string()),
            Lexeme::Iri("http://example.org/stats/gravity".to_string()),
            Lexeme::Unknown("-1.663E-4".to_string()),
            Lexeme::EndToken,
        ],
    );
}

#[test]
fn parse_prefixed_literal() {
    let input = vec![
        "@prefix : <http://example.org/stats> .",
        "<http://somecountry.example/census2007>",
        "    :isLandlocked \"false\"^^:boolean .",
    ]
    .join("\n");

    let tokens = tokenize(&input, 0, &mut LexerContext::new());

    assert_eq!(
        tokens,
        vec![
            Lexeme::Prefix(":".to_string(), "http://example.org/stats".to_string()),
            Lexeme::EndToken,
            Lexeme::Iri("http://somecountry.example/census2007".to_string()),
            Lexeme::PrefixedIri(":isLandlocked".to_string()),
            Lexeme::DataTypeLiteral("false".to_string(), ":boolean".to_string()),
            Lexeme::EndToken,
        ],
    );
}

#[test]
fn parse_prefixed_literal2() {
    let input = vec![
        "@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .",
        "@prefix show: <http://example.org/vocab/show/> .",
        "@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .",
        "",
        "show:218 rdfs:label \"That Seventies Show\"^^xsd:string . ",
    ]
    .join("\n");

    let tokens = tokenize(&input, 0, &mut LexerContext::new());

    assert_eq!(
        tokens,
        vec![
            Lexeme::Prefix(
                "rdfs:".to_string(),
                "http://www.w3.org/2000/01/rdf-schema#".to_string()
            ),
            Lexeme::EndToken,
            Lexeme::Prefix(
                "show:".to_string(),
                "http://example.org/vocab/show/".to_string()
            ),
            Lexeme::EndToken,
            Lexeme::Prefix(
                "xsd:".to_string(),
                "http://www.w3.org/2001/XMLSchema#".to_string()
            ),
            Lexeme::EndToken,
            Lexeme::PrefixedIri("show:218".to_string()),
            Lexeme::PrefixedIri("rdfs:label".to_string()),
            Lexeme::DataTypeLiteral("That Seventies Show".to_string(), "xsd:string".to_string()),
            Lexeme::EndToken,
        ],
    );
}

#[test]
fn parse_multiline() {
    let input = vec![
        "<http://somecountry.example/census2007> <http://example.org/stats/gravity> \"\"\"hello multi",
        "",
        " line \"\" literal\"\"\" .",
    ];

    let mut i = 0;
    let mut tokens: Vec<Lexeme> = vec![];
    let mut lexer_context = LexerContext::new();

    for line in input {
        let mut line_tokens = tokenize(&line, i, &mut lexer_context);
        tokens.append(&mut line_tokens);
        i += 1;
    }

    assert_eq!(
        tokens,
        vec![
            Lexeme::Iri("http://somecountry.example/census2007".to_string()),
            Lexeme::Iri("http://example.org/stats/gravity".to_string()),
            Lexeme::MultilineLiteral("hello multi\n\n line \"\" literal".to_string()),
            Lexeme::EndToken,
        ],
    );
}

#[test]
fn parse_short_multiline() {
    let input = vec![
        "<http://somecountry.example/census2007> <http://example.org/stats/gravity> \"\"\"short multi line\"\"\" .",
    ];

    let mut i = 0;
    let mut tokens: Vec<Lexeme> = vec![];
    let mut lexer_context = LexerContext::new();

    for line in input {
        let mut line_tokens = tokenize(&line, i, &mut lexer_context);
        tokens.append(&mut line_tokens);
        i += 1;
    }

    assert_eq!(
        tokens,
        vec![
            Lexeme::Iri("http://somecountry.example/census2007".to_string()),
            Lexeme::Iri("http://example.org/stats/gravity".to_string()),
            Lexeme::MultilineLiteral("short multi line".to_string()),
            Lexeme::EndToken,
        ],
    );
}

#[test]
fn parse_multiple_multilines() {
    let input = vec![
        "<http://somecountry.example/census2007> <http://example.org/stats/gravity> \"\"\"hello multi",
        "",
        " line \"\" literal\"\"\" .",
        "<http://somecountry.example/census2007> <http://example.org/stats/gravity> \"\"\"hello multi",
        "",
        " line 2\"\" literal\"\"\" .",
    ];

    let mut i = 0;
    let mut tokens: Vec<Lexeme> = vec![];
    let mut lexer_context = LexerContext::new();

    for line in input {
        let mut line_tokens = tokenize(&line, i, &mut lexer_context);
        tokens.append(&mut line_tokens);
        i += 1;
    }

    assert_eq!(
        tokens,
        vec![
            Lexeme::Iri("http://somecountry.example/census2007".to_string()),
            Lexeme::Iri("http://example.org/stats/gravity".to_string()),
            Lexeme::MultilineLiteral("hello multi\n\n line \"\" literal".to_string()),
            Lexeme::EndToken,
            Lexeme::Iri("http://somecountry.example/census2007".to_string()),
            Lexeme::Iri("http://example.org/stats/gravity".to_string()),
            Lexeme::MultilineLiteral("hello multi\n\n line 2\"\" literal".to_string()),
            Lexeme::EndToken,
        ],
    );
}
