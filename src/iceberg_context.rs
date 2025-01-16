//! Iceberg Context

use http::Uri;
use iceberg::Catalog;
use iceberg_catalog_rest::{RestCatalog, RestCatalogConfig};

/// Iceberg Context
#[derive(Debug)]
pub struct IcebergContext {
    /// Iceberg Catalog
    pub catalog: Box<dyn Catalog>,
}

impl IcebergContext {
    /// Create a new Iceberg Context from a Uri
    pub fn new(uri: Uri) -> Self {
        let mut uri_str = uri.to_string();
        uri_str.pop();

        let config = RestCatalogConfig::builder().uri(uri_str).build();

        let rest_catalog = RestCatalog::new(config);

        Self {
            catalog: Box::new(rest_catalog),
        }
    }
}
