use std::{collections::HashMap, fmt::Debug};

use serde::{Deserialize, Serialize};

pub mod stop_query;

#[derive(Serialize, Deserialize, Debug)]
pub struct GraphQLResponse<T> {
    pub data: Option<T>,
    pub errors: Option<Vec<GraphQLError>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GraphQLError {
    message: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Cursor {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryArgs {
    #[serde(rename = "where")]
    pub wheres: HashMap<String, serde_json::Value>,
    #[serde(rename = "orderBy")]
    pub order_by: Vec<HashMap<String, String>>,
    pub take: Option<u64>,
    pub skip: Option<u64>,
    pub cursor: Option<Cursor>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MutationArgs<D, W> {
    pub data: Vec<MutationsData<D, W>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MutationsData<D, W> {
    pub data: D,
    #[serde(rename = "where")]
    pub wheres: W,
}

impl Default for QueryArgs {
    fn default() -> Self {
        let mut stop_order = HashMap::new();
        stop_order.insert("stopNumber".to_string(), "asc".to_string());

        QueryArgs {
            wheres: HashMap::new(),
            take: Some(250),
            skip: Some(0),
            cursor: None,
            order_by: vec![stop_order],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::query::Cursor;

    use super::QueryArgs;
    use serde_json::{Value, json};
    use std::collections::HashMap;

    #[test]
    fn json_serialization() {
        let mut wheres = HashMap::new();
        wheres.insert("name".to_string(), json!("Alice"));
        wheres.insert("age".to_string(), json!(30));
        let mut order_by = HashMap::new();
        order_by.insert("id".to_string(), "asc".to_string());

        let query_args = QueryArgs {
            wheres,
            take: Some(10),
            skip: Some(5),
            order_by: vec![order_by],
            cursor: Some(Cursor {
                id: "abc123".to_string(),
            }),
        };

        let serialized = serde_json::to_string(&query_args).expect("Failed to serialize");
        println!("Serialized JSON: {}", serialized);

        let deserialized: QueryArgs =
            serde_json::from_str(&serialized).expect("Failed to deserialize");

        // Assertions
        assert_eq!(deserialized.take, Some(10));
        assert_eq!(deserialized.skip, Some(5));
        assert_eq!(
            deserialized.cursor,
            Some(Cursor {
                id: "abc123".to_string()
            })
        );
        assert_eq!(
            deserialized.wheres.get("name"),
            Some(&Value::String("Alice".to_string()))
        );
        assert_eq!(
            deserialized.wheres.get("age"),
            Some(&Value::Number(30.into()))
        );
    }
}
