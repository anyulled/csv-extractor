use anyhow::{Context, Result};
use duckdb::Connection;
use std::env;
use std::path::Path;

fn main() -> Result<()> {
    let input_file_path = parse_args()?;
    validate_input_file(&input_file_path)?;

    let output_filename = generate_output_filename(&input_file_path)?;

    extract_columns(&input_file_path, &output_filename)?;

    Ok(())
}

fn parse_args() -> Result<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        anyhow::bail!("Usage: {} <input_csv_file>", args[0]);
    }
    Ok(args[1].clone())
}

fn validate_input_file(input_path: &str) -> Result<()> {
    if !input_path.to_lowercase().ends_with(".csv") {
        anyhow::bail!("Input file must be a CSV file");
    }
    Ok(())
}

fn generate_output_filename(input_path: &str) -> Result<String> {
    let path = Path::new(input_path);
    let file_stem = path
        .file_stem()
        .context("Invalid file name")?
        .to_string_lossy();
    let extension = path.extension().unwrap_or_default().to_string_lossy();

    if extension.is_empty() {
        Ok(format!("{}-extracted", file_stem))
    } else {
        Ok(format!("{}-extracted.{}", file_stem, extension))
    }
}

fn extract_columns(input_path: &str, output_path: &str) -> Result<()> {
    let connection = Connection::open_in_memory()?;

    let query = format!(
        r#"COPY (SELECT array_to_string(list_transform(str_split(name, ' '), x -> upper(substring(x, 1, 1)) || lower(substring(x, 2))), ' ') AS name, COALESCE(NULLIF("DNI/NIE Identification number", ''), 'N/A') AS "identification number" FROM read_csv_auto('{}') WHERE approval_status = 'approved') TO '{}' (HEADER, DELIMITER ',')"#,
        input_path, output_path
    );

    connection
        .execute_batch(&query)
        .context("Failed to execute DuckDB query for extraction")?;

    println!("Successfully generated {}", output_path);

    Ok(())
}
