use tanic::config::CatalogConnectionDetails;

pub(crate) struct AddConnectionDialogState {
    name: String,
    uri: String,
}

pub(crate) enum TanicUiState {
    Initializing,
    ConnectionPrompt(ViewConnectionPromptState),
    ConnectionList(ViewConnectionListState),
    NamespaceTreeView(ViewNamespaceTreeViewState),
}

pub(crate) struct ViewConnectionPromptState {
    connection_uri: String,
}

pub(crate) struct ViewConnectionListState {
    connections: Vec<CatalogConnectionDetails>,
    add_connection_dialog_open: bool,
    add_connection_dialog_name: String,
    add_connection_dialog_uri: String,
}

pub(crate) struct ViewNamespaceTreeViewState {

}