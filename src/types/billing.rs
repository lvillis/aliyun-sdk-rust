use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryAccountBalanceParams {}

impl QueryAccountBalanceParams {
    pub(crate) fn into_query(self) -> BTreeMap<String, String> {
        BTreeMap::new()
    }
}
