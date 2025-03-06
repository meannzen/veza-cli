# veza-cli Usage Guide

```sh
export API_URL="https://your-graphql-api.com/graphql"
export API_TOKEN="your-secret-token"
```

## Overview
`veza-cli` is a command-line tool designed to manage data models using Excel and various formatting options.

## Installation
Ensure you have Rust installed. If not, install it from [rustup.rs](https://rustup.rs/). Then, you can build and install `veza-cli`:

```sh
cargo install --path .
```

## Commands

### Main Command Structure
```sh
veza-cli <MODEL_COMMAND>
```

### Stop Model Commands
The `stop` command allows exporting and formatting stop data.

#### Export Stops
Exports stop data to an Excel file.

```sh
veza-cli stop export --file stops.xlsx
```
- `-f, --file` (optional): Output file name (default: `output.xlsx`).

#### Format Stops
Format stop data from different sources and optionally save to an Excel file.

##### Pull Stops from Backend
Fetches stops from the backend API, formats them, and optionally updates the backend.

```sh
veza-cli stop format pull --output formatted.xlsx --update-backend
```
- `-o, --output` (optional): Output file name (default: `formatted_output.xlsx`).
- `-u, --update-backend`: Whether to update the backend after formatting.

##### Read Stops from Excel
Reads stop data from an Excel file, formats it, and optionally updates the backend.

```sh
veza-cli stop format read-xlsx --file input.xlsx --output formatted.xlsx --update-backend
```
- `-f, --file` (optional): Path to the input Excel file (default: `input.xlsx`).
- `-o, --output` (optional): Output file name (default: `formatted_output.xlsx`).
- `-u, --update-backend`: Whether to update the backend after formatting.

##### Stop ID Generation
Generates stop IDs based on a pattern and organization ID.

```sh
veza-cli stop format stop-id --pattern ST123456 --organization 123
```
- `-p, --pattern` (optional): Pattern for stop IDs (default: `ST000000`).
- `-o, --organization`: Organization ID (required).

## Examples

### Export Stops to a Custom File
```sh
veza-cli stop export --file my_stops.xlsx
```

### Format Stops from Excel and Update Backend
```sh
veza-cli stop format read-xlsx --file my_input.xlsx --output my_output.xlsx --update-backend
```

### Pull Stops, Format, and Save
```sh
veza-cli stop format pull --output new_stops.xlsx
```

## Conclusion
`veza-cli` simplifies managing stop data by allowing you to export, format, and update stops efficiently through a CLI interface.


