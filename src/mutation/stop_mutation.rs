use crate::{query::MutationArgs, service::graphql::GraphQLService};
use serde::{Deserialize, Serialize}; 
use serde_json::json;

pub async fn stop_mutation<'de, D, W>(
    data: MutationArgs<D, W>,
    service: &GraphQLService,
) -> Result<(), Box<dyn std::error::Error>>
where
    D: Serialize + Deserialize<'de>,
    W: Serialize + Deserialize<'de>, // Add this trait bound
{
    let mutation = r#"
        mutation UpdateStops($data: [StopUpdateArgs!]!) {
            updateStops(data: $data) {
                id
                stopId
                position
                latitude
                longitude
            }
        }
    "#;
    let request_body: serde_json::Value = json!({ "query": mutation, "variables": data });
    let _ = service.execute(request_body).await?;
    Ok(())
}
