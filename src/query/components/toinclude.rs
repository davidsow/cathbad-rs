use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ToIncludeType {
    All,
    None,
    List,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ToInclude {
    All {
        #[serde(rename = "type")]
        type_: ToIncludeType,
    },
    None {
        #[serde(rename = "type")]
        type_: ToIncludeType,
    },
    List {
        #[serde(rename = "type")]
        type_: ToIncludeType,
        columns: Vec<String>,
    },
}
