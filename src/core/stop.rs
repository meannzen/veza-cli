use reqwest::Client;
use tracing::info;

use crate::{
    cli::FormatCommand, models::stop::Stop, service::geocoding_service::GeocodingService,
    utils::xlsx::read_xlsx,
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
    }
    Ok(())
}
