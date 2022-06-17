use crate::ast::{Attr, DeclId, PathId, Statement};

const INDENT: &str = "  ";

pub fn graph_to_str(ast: &[Statement], directed: bool, autolabel: bool, shape: String) -> String {
    format!(
        "{} {{\n{}overlap=false;\n{}node [{}shape={}, style=filled, fillcolor=\"#ffffff00\"];\n{}\n}}",
        if directed { "digraph" } else { "graph" },
        INDENT,
        INDENT,
        if autolabel { "" } else { "label=\"\", " },
        shape,
        ast.iter()
            .map(|s| statement_to_str(s, directed, 1))
            .collect::<Vec<String>>()
            .join("\n")
    )
}

fn statement_to_str(s: &Statement, directed: bool, depth: usize) -> String {
    match s {
        Statement::Decl { id, attrs } => decl_to_str(id, attrs, depth),
        Statement::Path { ids, attrs } => path_to_str(ids, attrs, directed, depth),
        Statement::Subgraph(v) => subgraph_to_str(v, directed, depth),
    }
}

fn subgraph_to_str(v: &[Statement], directed: bool, depth: usize) -> String {
    let indent = INDENT.repeat(depth);
    let stmt = v
        .iter()
        .map(|s| statement_to_str(s, directed, depth + 1))
        .collect::<Vec<String>>()
        .join("\n");
    format!("{}{{\n{}\n{}}}", indent, stmt, indent)
}

fn decl_to_str(id: &DeclId, attrs: &Option<Vec<Attr>>, depth: usize) -> String {
    let indent = INDENT.repeat(depth);
    let stmt_id = decl_id_to_str(id);
    match attrs {
        None => format!("{}{}", indent, stmt_id),
        Some(attrs) => format!(
            "{}{} [{}]",
            indent,
            stmt_id,
            attrs
                .iter()
                .map(attr_to_str)
                .collect::<Vec<String>>()
                .join(", ")
        ),
    }
}

fn path_to_str(ids: &[PathId], attrs: &Option<Vec<Attr>>, directed: bool, depth: usize) -> String {
    let indent = INDENT.repeat(depth);
    let stmt_id = ids
        .iter()
        .map(path_id_to_str)
        .collect::<Vec<String>>()
        .join(if directed { " -> " } else { " -- " });
    match attrs {
        None => format!("{}{}", indent, stmt_id),
        Some(attrs) => format!(
            "{}{} [{}]",
            indent,
            stmt_id,
            attrs
                .iter()
                .map(attr_to_str)
                .collect::<Vec<String>>()
                .join(", ")
        ),
    }
}

fn decl_id_to_str(id: &DeclId) -> String {
    match id {
        DeclId::Node(s) => s.clone(),
        DeclId::Keyword(s) => s.clone(),
    }
}

fn path_id_to_str(id: &PathId) -> String {
    match id {
        PathId::Node(s) => s.clone(),
        PathId::Subgraph(v) => format!(
            "{{ {} }}",
            v.iter()
                .map(|s| statement_to_str(s, false, 0))
                .collect::<Vec<String>>()
                .join(" ")
        ),
    }
}

fn attr_to_str(attr: &Attr) -> String {
    format!("{}={}", attr.key, attr.value)
}
