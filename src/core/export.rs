use std::error::Error;

use rust_xlsxwriter::Workbook;
use tracing::info;

use crate::{
    config::Config,
    models::traits::Model,
    query::{Cursor, QueryArgs, stop_query::fetch_stops},
    service::graphql::GraphQLService,
};

pub fn export_to_excel<T: Model>(items: Vec<T>, file_name: &str) -> Result<(), Box<dyn Error>> {
    info!(
        "Exporting {} {}s to {}",
        items.len(),
        T::display_name(),
        file_name
    );

    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    for (col, &header) in T::headers().iter().enumerate() {
        worksheet.write_string(0, col as u16, header)?;
    }

    for (row, item) in items.iter().enumerate() {
        let row = row as u32 + 1;
        for (col, value) in item.to_row().iter().enumerate() {
            worksheet.write_string(row, col as u16, value)?;
        }
    }

    workbook.save(file_name)?;
    info!(
        "Successfully exported {} {}s to '{}'",
        items.len(),
        T::display_name(),
        file_name
    );

    Ok(())
}

pub async fn export_stops_to_excel(
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

    export_to_excel(all_stops, &file_name)?;
    Ok(())
}
