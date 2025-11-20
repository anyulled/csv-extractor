# CSV Extractor

A 🦀 Rust application that extracts and transforms specific data from Luma CSV export files using 🦆 DuckDB.

## Features

- **Extraction**: Extracts `name` and `DNI/NIE Identification number` columns.
- **Filtering**: Filters records to include only those with `approval_status` set to `approved`.
- **Transformation**:
    - Converts names to **Title Case** (e.g., "JOHN DOE" -> "John Doe").
    - Renames the identification column to `identification number`.
    - Replaces empty or missing identification numbers with `N/A`.
- **Validation**: Ensures the input file has a `.csv` extension.
- **Output**: Generates a new CSV file with the suffix `-extracted` (e.g., `input.csv` -> `input-extracted.csv`).

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)

## Installation

1. Clone the repository:
   ```bash
   git clone git@github.com:anyulled/csv-extractor.git
   cd csv-extractor
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

Run the application using `cargo run` and provide the path to your CSV file:

```bash
cargo run -- <path_to_csv_file>
```

### Example

```bash
cargo run -- data/guests.csv
```

This will generate a file named `data/guests-extracted.csv` containing the processed data.

## Testing

To run the integration tests:

```bash
cargo test
```
