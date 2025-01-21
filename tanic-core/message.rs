#[derive(Clone, Debug)]
pub struct NamespaceDeets {
    pub parts: Vec<String>,
    pub name: String,
    pub table_count: usize,
}

#[derive(Clone, Debug)]
pub struct TableDeets {
    pub namespace: Vec<String>,
    pub name: String,
    pub row_count: usize,
}
