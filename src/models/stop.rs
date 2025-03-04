use serde::{Deserialize, Serialize};

use crate::utils::xlsx::FromExcelRow;

use super::traits::Model;

#[derive(Serialize, Deserialize, Debug, Clone)]
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

impl FromExcelRow for Stop {
    fn from_row(
        row: &[calamine::Data],
        header_map: &std::collections::HashMap<String, usize>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let id_idx = *header_map.get("ID").ok_or("Missing 'ID' column")?;
        let stop_id_idx = *header_map.get("StopID").ok_or("Missing 'StopID' column")?;
        let position_idx = *header_map
            .get("Address")
            .ok_or("Missing 'Address' column")?;
        let latitude_idx = *header_map
            .get("Latitude")
            .ok_or("Missing 'Latitude' column")?;
        let longitude_idx = *header_map
            .get("Longtitude")
            .ok_or("Missing 'Longtitude' column")?;

        if row.len() <= id_idx
            || row.len() <= stop_id_idx
            || row.len() <= position_idx
            || row.len() <= latitude_idx
            || row.len() <= longitude_idx
        {
            return Err("Row has insufficient columns".into());
        }

        Ok(Stop {
            id: row[id_idx].to_string(),
            stop_id: row[stop_id_idx].to_string(),
            position: row[position_idx].to_string(),
            latitude: row[latitude_idx].to_string(),
            longitude: row[longitude_idx].to_string(),
        })
    }
}
