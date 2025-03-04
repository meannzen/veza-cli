use std::{collections::HashMap, error::Error, path::Path};

use calamine::{Data, Reader, Xlsx, open_workbook};
use rust_xlsxwriter::Workbook;
use tracing::{info, warn};

use crate::models::traits::Model;

pub fn write_xlsx<T: Model>(items: Vec<T>, file_name: &str) -> Result<(), Box<dyn Error>> {
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

pub fn read_xlsx<T: Model + FromExcelRow>(file_path: &str) -> Result<Vec<T>, Box<dyn Error>> {
    let path = Path::new(file_path);
    let mut workbook: Xlsx<_> = open_workbook(path)
        .map_err(|e| format!("Failed  to open Excel file  '{}': {}", file_path, e))?;

    let sheet = workbook
        .worksheet_range_at(0)
        .ok_or("No sheets found in Excel file")?
        .map_err(|e| format!("Failed to read sheet: {}", e))?;

    let mut rows = sheet.rows();
    let headers = rows.next().ok_or("No header row found")?;
    let expected_header = T::headers();

    let header_map: HashMap<String, usize> = headers
        .iter()
        .enumerate()
        .map(|(i, h)| (h.to_string(), i))
        .collect();

    for expected in expected_header.iter() {
        if !header_map.contains_key(*expected) {
            return Err(format!(
                "Missing expected header '{}' in '{}'.  Found: {:?}",
                expected,
                file_path,
                header_map.keys().collect::<Vec<_>>()
            )
            .into());
        }
    }

    let mut items = Vec::new();
    for (i, row) in rows.enumerate() {
        match T::from_row(row, &header_map) {
            Ok(item) => items.push(item),
            Err(e) => warn!("Row {} failed to pasrse: {}", i + 2, e),
        }
    }
    if items.is_empty() {
        warn!("No Valid items found  in '{}'", file_path);
    } else {
        info!("Read {} items from '{}'", items.len(), file_path);
    }

    Ok(items)
}

pub trait FromExcelRow: Sized {
    fn from_row(row: &[Data], header_map: &HashMap<String, usize>) -> Result<Self, Box<dyn Error>>;
}
