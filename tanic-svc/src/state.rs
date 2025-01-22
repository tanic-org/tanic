use tanic_core::config::ConnectionDetails;
use tanic_core::message::{NamespaceDeets, TableDeets};

#[derive(Debug)]
pub enum TanicAction {
    Exit,

    ConnectTo(ConnectionDetails),

    RetrievedNamespaceList(Vec<NamespaceDeets>),
    FocusPrevNamespace,
    FocusNextNamespace,
    SelectNamespace,

    RetrievedTableList(NamespaceDeets, Vec<TableDeets>),
    EnrichedTableDetails(),
    FocusPrevTable,
    FocusNextTable,
    SelectTable,
    LeaveNamespace,
}

#[derive(Clone, Debug, Default)]
pub enum TanicAppState {
    #[default]
    Initializing,
    ConnectingTo(ConnectionDetails),
    ViewingNamespacesList(ViewingNamespacesListState),
    RetrievingTableList(ViewingNamespacesListState),
    ViewingTablesList(ViewingTablesListState),
    Exiting,
}

#[derive(Clone, Debug)]
pub struct ViewingNamespacesListState {
    pub namespaces: Vec<NamespaceDeets>,
    pub selected_idx: Option<usize>,
}

#[derive(Clone, Debug)]
pub struct ViewingTablesListState {
    pub namespaces: ViewingNamespacesListState,
    pub namespace: NamespaceDeets,
    pub tables: Vec<TableDeets>,
    pub selected_idx: Option<usize>,
}

impl TanicAppState {
    pub(crate) fn reduce(self, action: TanicAction) -> Self {
        match (action, &self) {
            (TanicAction::Exit, _) => TanicAppState::Exiting,

            (TanicAction::ConnectTo(conn_details), _) => TanicAppState::ConnectingTo(conn_details),

            (TanicAction::RetrievedNamespaceList(namespaces), _) => {
                let selected_idx = if namespaces.is_empty() { None } else { Some(0) };

                TanicAppState::ViewingNamespacesList(ViewingNamespacesListState {
                    namespaces,
                    selected_idx,
                })
            }

            (
                TanicAction::FocusPrevNamespace,
                TanicAppState::ViewingNamespacesList(ViewingNamespacesListState {
                    namespaces,
                    selected_idx,
                }),
            ) => {
                let selected_idx = selected_idx.map(|selected_idx| {
                    if selected_idx == 0 {
                        namespaces.len() - 1
                    } else {
                        selected_idx - 1
                    }
                });

                TanicAppState::ViewingNamespacesList(ViewingNamespacesListState {
                    namespaces: namespaces.clone(),
                    selected_idx,
                })
            }

            (
                TanicAction::FocusNextNamespace,
                TanicAppState::ViewingNamespacesList(ViewingNamespacesListState {
                    namespaces,
                    selected_idx,
                }),
            ) => {
                let selected_idx = selected_idx.map(|selected_idx| {
                    if selected_idx == namespaces.len() - 1 {
                        0
                    } else {
                        selected_idx + 1
                    }
                });

                TanicAppState::ViewingNamespacesList(ViewingNamespacesListState {
                    namespaces: namespaces.clone(),
                    selected_idx,
                })
            }

            (
                TanicAction::SelectNamespace,
                TanicAppState::ViewingNamespacesList(ViewingNamespacesListState {
                    selected_idx,
                    namespaces,
                }),
            ) => TanicAppState::RetrievingTableList(ViewingNamespacesListState {
                selected_idx: *selected_idx,
                namespaces: namespaces.clone(),
            }),

            (
                TanicAction::RetrievedTableList(namespace, tables),
                TanicAppState::RetrievingTableList(ViewingNamespacesListState {
                    selected_idx: namespace_selected_idx,
                    namespaces,
                }),
            ) => {
                let table_selected_idx = if tables.is_empty() { None } else { Some(0) };

                TanicAppState::ViewingTablesList(ViewingTablesListState {
                    namespaces: ViewingNamespacesListState {
                        selected_idx: *namespace_selected_idx,
                        namespaces: namespaces.clone(),
                    },
                    namespace,
                    tables,
                    selected_idx: table_selected_idx,
                })
            }

            (
                TanicAction::FocusPrevTable,
                TanicAppState::ViewingTablesList(ViewingTablesListState {
                    namespaces,
                    namespace,
                    tables,
                    selected_idx,
                }),
            ) => {
                let selected_idx = selected_idx.map(|selected_idx| {
                    if selected_idx == 0 {
                        tables.len() - 1
                    } else {
                        selected_idx - 1
                    }
                });

                TanicAppState::ViewingTablesList(ViewingTablesListState {
                    namespaces: namespaces.clone(),
                    namespace: namespace.clone(),
                    tables: tables.clone(),
                    selected_idx,
                })
            }

            (
                TanicAction::FocusNextTable,
                TanicAppState::ViewingTablesList(ViewingTablesListState {
                    namespaces,
                    namespace,
                    tables,
                    selected_idx,
                }),
            ) => {
                let selected_idx = selected_idx.map(|selected_idx| {
                    if selected_idx == tables.len() - 1 {
                        0
                    } else {
                        selected_idx + 1
                    }
                });

                TanicAppState::ViewingTablesList(ViewingTablesListState {
                    namespaces: namespaces.clone(),
                    namespace: namespace.clone(),
                    tables: tables.clone(),
                    selected_idx,
                })
            }

            (TanicAction::SelectTable, _) => self,

            (
                TanicAction::LeaveNamespace,
                TanicAppState::ViewingTablesList(ViewingTablesListState { namespaces, .. }),
            ) => TanicAppState::ViewingNamespacesList(namespaces.clone()),

            _ => self,
        }
    }
}
