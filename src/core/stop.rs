use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::info;

use crate::{
    cli::{FormatCommand, StopIDArgs},
    models::stop::Stop,
    mutation::stop_mutation,
    query::{MutationArgs, MutationsData},
    service::geocoding_service::GeocodingService,
    utils::{generate_id::generate_stop_id, xlsx::read_xlsx},
};
use std::error::Error;

use crate::{
    config::Config,
    query::{Cursor, QueryArgs, stop_query::fetch_stops},
    service::graphql::GraphQLService,
    utils::xlsx::write_xlsx,
};

pub async fn process_export_stops_to_excel(
    file_name: String,
    config: &Config,
) -> Result<(), Box<dyn Error>> {
    let service = GraphQLService::new(config);
    let mut args = QueryArgs::default();
    let mut all_stops: Vec<crate::models::stop::Stop> = Vec::new();

    loop {
        let response: crate::query::GraphQLResponse<crate::query::stop_query::StopResponse> =
            fetch_stops(&args, &service).await?;

        // Check if data exists
        if let Some(stop_response) = response.data {
            let count = stop_response.stops.len();
            all_stops.extend(stop_response.stops);

            // If we received fewer results than `take`, we are done
            if count < args.take.unwrap_or(250) as usize {
                break;
            }
            args.skip = Some(1);
            args.cursor = Some(Cursor {
                id: all_stops.last().unwrap().id.clone(),
            })
        } else {
            break;
        }
    }

    write_xlsx(all_stops, &file_name)?;
    Ok(())
}

pub async fn process_format_command(
    command: FormatCommand,
    config: &Config,
) -> Result<(), Box<dyn Error>> {
    match command {
        FormatCommand::Pull(_) => {
            println!("x");
        }

        FormatCommand::ReadXlsx(args) => match args.update_backend {
            true => {}
            false => {
                let mut stops: Vec<Stop> = read_xlsx(&args.file_path)?;
                info!("Read {} stops from {}", stops.len(), args.file_path);
                let client = Client::new();
                let geocoding_service = GeocodingService::new(client, config);
                geocoding_service.geocode_stops(&mut stops).await?;
                info!("Writing formatted stops to {}", args.output_file);
                write_xlsx(stops, &args.output_file)?;
            }
        },

        FormatCommand::StopID(args) => format_stop_id(args, config).await?,
    }
    Ok(())
}

#[derive(Deserialize, Serialize, Debug)]
struct StopData {
    #[serde(rename = "stopId")]
    stop_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct StopWhereAge {
    id: String,
}

type StopUpdateData = MutationArgs<StopData, StopWhereAge>;
async fn format_stop_id(stop_args: StopIDArgs, config: &Config) -> Result<(), Box<dyn Error>> {
    let service = GraphQLService::new(config);
    let mut args = QueryArgs::default();

    args.wheres.insert(
        "organizations".to_string(),
        json!({
                "some": {
                    "id": {
                        "equals": stop_args.organization_id
                    }
                }
            }
        ),
    );

    let mut index = 0;

    loop {
        let response: crate::query::GraphQLResponse<crate::query::stop_query::StopResponse> =
            fetch_stops(&args, &service).await?;

        let mut update_stops: StopUpdateData = StopUpdateData { data: vec![] };

        // Check if data exists
        if let Some(stop_response) = response.data {
            let count = stop_response.stops.len();

            let stops: Vec<Stop> = stop_response.stops;

            for stop in stops.iter() {
                let id = generate_stop_id(&stop_args.pattern, index);
                index += 1;
                update_stops.data.push(MutationsData {
                    data: StopData { stop_id: id },
                    wheres: StopWhereAge {
                        id: stop.id.clone(),
                    },
                });
            }
            info!("Updating {}", count);
            let s = stop_mutation::stop_mutation(update_stops, &service).await;
            match s {
                Ok(_) => {}
                Err(e) => info!("error {}", e),
            }

            info!("end update stops {}", count);

            // If we received fewer results than `take`, we are done
            if count < args.take.unwrap_or(250) as usize {
                break;
            }
            args.skip = Some(1);
            args.cursor = Some(Cursor {
                id: stops.last().unwrap().id.clone(),
            });
        } else {
            break;
        }
    }
    Ok(())
}
