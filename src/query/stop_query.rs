use serde::{Deserialize, Serialize};
use serde_json::json;

use super::{GraphQLResponse, QueryArgs};
use crate::{models::stop::Stop, service::graphql::GraphQLService};

#[derive(Serialize, Deserialize, Debug)]
pub struct StopResponse {
    pub stops: Vec<Stop>,
}

pub async fn fetch_stops(
    args: &QueryArgs,
    service: &GraphQLService,
) -> Result<GraphQLResponse<StopResponse>, Box<dyn std::error::Error>> {
    let query = r#"
      query Stops(
        $orderBy: [StopOrderByInput!]!
        $take: Int!
        $skip: Int!
        $cursor: StopWhereUniqueInput
        $where: StopWhereInput!
      ) {
        stops(
            orderBy: $orderBy
            take: $take
            skip: $skip
            cursor: $cursor
            where: $where
        ) {
            id
            stopId
            position
            latitude
            longitude
        }
      }
    "#;

    let request_body = json!({ "query": query, "variables": args });

    let response = service.execute(request_body).await?;
    let stop_response: GraphQLResponse<StopResponse> =
        serde_json::from_value(response).map_err(|e| format!("Failed to parse response: {}", e))?;

    Ok(stop_response)
}
