use tree_sitter::{Parser, Query, QueryCursor};

fn main() {
    let language = tree_sitter_rust::language();
    let source = "let rofl = lol + lmao;";
    let query_string = "(identifier) @ident @tnedi (binary_expression) @bin";

    let mut parser = Parser::new();
    parser.set_language(language).unwrap();

    let tree = parser.parse(source, None).unwrap();
    let query = Query::new(language, query_string).unwrap();
    let mut query_cursor = QueryCursor::new();
    let captures: Vec<_> = query_cursor
        .captures(&query, tree.root_node(), source.as_bytes())
        .map(|(cap, _)| cap)
        .flat_map(|cap| cap.captures)
        .cloned() // comment this to trigger UB
        .collect();

    dbg!(captures);
}
