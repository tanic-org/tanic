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

    FocusPrevTable,
    FocusNextTable,
    SelectTable,
}

#[derive(Clone, Debug, Default)]
pub enum TanicAppState {
    #[default]
    Initializing,
    ConnectingTo(ConnectionDetails),
    ViewingNamespacesList(ViewingNamespacesListState),
    ViewingTablesList(ViewingTablesListState),
    Exiting,
}

#[derive(Clone, Debug)]
pub struct ViewingNamespacesListState {
    pub namespaces: Vec<NamespaceDeets>,
    pub selected_idx: usize,
}

#[derive(Clone, Debug)]
pub struct ViewingTablesListState {
    pub namespace: String,
    pub tables: Vec<TableDeets>,
    pub selected_idx: usize,
}

impl TanicAppState {
    pub(crate) fn reduce(self, action: TanicAction) -> Self {
        match (action, &self) {
            (TanicAction::Exit, _) => TanicAppState::Exiting,

            (TanicAction::ConnectTo(conn_details), _) => TanicAppState::ConnectingTo(conn_details),

            (TanicAction::RetrievedNamespaceList(namespaces), _) => {
                TanicAppState::ViewingNamespacesList(ViewingNamespacesListState {
                    namespaces,
                    selected_idx: 0,
                })
            }

            (
                TanicAction::FocusPrevNamespace,
                TanicAppState::ViewingNamespacesList(ViewingNamespacesListState {
                    namespaces,
                    selected_idx,
                }),
            ) => {
                let selected_idx = if *selected_idx == 0 {
                    namespaces.len() - 1
                } else {
                    *selected_idx - 1
                };

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
                let selected_idx = if *selected_idx == namespaces.len() - 1 {
                    0
                } else {
                    *selected_idx + 1
                };

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
            ) => TanicAppState::ViewingTablesList(ViewingTablesListState {
                namespace: namespaces[*selected_idx].name.clone(),
                tables: vec![],
                selected_idx: 0,
            }),

            (
                TanicAction::FocusPrevTable,
                TanicAppState::ViewingTablesList(ViewingTablesListState {
                    namespace,
                    tables,
                    selected_idx,
                }),
            ) => {
                let selected_idx = if *selected_idx == 0 {
                    tables.len() - 1
                } else {
                    *selected_idx - 1
                };

                TanicAppState::ViewingTablesList(ViewingTablesListState {
                    namespace: namespace.clone(),
                    tables: tables.clone(),
                    selected_idx,
                })
            }

            (
                TanicAction::FocusNextTable,
                TanicAppState::ViewingTablesList(ViewingTablesListState {
                    namespace,
                    tables,
                    selected_idx,
                }),
            ) => {
                let selected_idx = if *selected_idx == tables.len() - 1 {
                    0
                } else {
                    *selected_idx + 1
                };

                TanicAppState::ViewingTablesList(ViewingTablesListState {
                    namespace: namespace.clone(),
                    tables: tables.clone(),
                    selected_idx,
                })
            }

            (TanicAction::SelectTable, _) => self,

            _ => self,
        }
    }
}
