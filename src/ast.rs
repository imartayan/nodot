#[derive(Debug)]
pub enum Statement {
    Decl {
        id: DeclId,
        attrs: Option<Vec<Attr>>,
    },
    Path {
        ids: Vec<PathId>,
        attrs: Option<Vec<Attr>>,
    },
    Subgraph(Vec<Statement>),
}

#[derive(Debug)]
pub enum DeclId {
    Node(String),
    Keyword(String),
}

#[derive(Debug)]
pub enum PathId {
    Node(String),
    Subgraph(Vec<Statement>),
}

#[derive(Debug)]
pub struct Attr {
    pub key: String,
    pub value: String,
}
