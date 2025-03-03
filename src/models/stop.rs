use serde::{Deserialize, Serialize};

use super::traits::Model;

#[derive(Serialize, Deserialize, Debug)]
pub struct Stop {
    pub id: String,
    pub position: String,
    pub latitude: String,
    pub longitude: String,
    #[serde(rename = "stopId")]
    pub stop_id: String,
}

impl Model for Stop {
    fn id(&self) -> &str {
        &self.id
    }

    fn display_name() -> &'static str {
        "Stop"
    }

    fn headers() -> Vec<&'static str> {
        vec!["ID", "StopID", "Address", "Latitude", "Longtitude"]
    }
    fn to_row(&self) -> Vec<String> {
        vec![
            self.id.clone(),
            self.stop_id.clone(),
            self.position.clone(),
            self.latitude.clone(),
            self.longitude.clone(),
        ]
    }
}
