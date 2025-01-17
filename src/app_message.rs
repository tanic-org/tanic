pub enum AppMessage {

    // Update of list of namespaces for current location
    NamespaceNameList(Vec<String>),

    NavigateUp,

    NavigateChildNamespace(String),

    TableNameList(String),
}