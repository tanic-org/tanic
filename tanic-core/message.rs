use crate::config::ConnectionDetails;
use std::sync::{Arc, RwLock};

#[derive(Debug)]
#[non_exhaustive]
pub enum TanicMessage {
    Exit,

    ConnectTo(ConnectionDetails),

    // Update of list of namespaces for current location
    ShowNamespaces(Arc<RwLock<Vec<NamespaceDeets>>>),

    NavigateUp,

    NavigateChildNamespace(String),

    TableNameList(String),
}

#[derive(Debug)]
pub struct NamespaceDeets {
    pub parts: Vec<String>,
    pub name: String,
}
