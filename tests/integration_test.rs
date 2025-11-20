use std::fs;
use std::path::Path;
use std::process::Command;

#[test]
fn test_csv_extraction() -> anyhow::Result<()> {
    let input_path = "test_input.csv";
    let output_path = "test_input-extracted.csv";

    if Path::new(output_path).exists() {
        fs::remove_file(output_path)?;
    }

    let csv_content = r#"api_id,name,first_name,last_name,email,phone_number,created_at,approval_status,checked_in_at,custom_source,qr_code_url,amount,amount_tax,amount_discount,currency,coupon_code,eth_address,solana_address,survey_response_rating,survey_response_feedback,ticket_type_id,ticket_name,"DNI/NIE Identification number "
1,john doe,John,Doe,john@example.com,123456789,2023-01-01,approved,2023-01-02,web,http://qr,100,10,0,USD,SAVE10,0x123,sol123,5,Great,1,VIP,12345678A
2,Jane Smith,Jane,Smith,jane@example.com,987654321,2023-01-03,pending,,app,http://qr2,50,5,0,EUR,,0x456,sol456,4,Good,2,General,
3,BOB JONES,Bob,Jones,bob@example.com,111222333,2023-01-04,approved,,web,http://qr3,100,10,0,USD,,0x789,sol789,3,Okay,1,VIP,
"#;

    fs::write(input_path, csv_content)?;

    let status = Command::new("cargo")
        .args(&["run", "--", input_path])
        .status()?;

    assert!(status.success());

    assert!(Path::new(output_path).exists());

    let extracted_content = fs::read_to_string(output_path)?;
    let lines: Vec<&str> = extracted_content.lines().collect();

    // Header + 2 approved rows (John Doe, Bob Jones). Jane Smith (pending) should be excluded.
    assert_eq!(lines.len(), 3);

    assert!(lines[0].contains("name"));
    assert!(lines[0].contains("identification number"));

    assert!(lines[1].contains("John Doe"));
    assert!(lines[1].contains("12345678A"));

    assert!(lines[2].contains("Bob Jones"));
    assert!(lines[2].contains("N/A"));

    fs::remove_file(input_path)?;
    fs::remove_file(output_path)?;

    Ok(())
}

#[test]
fn test_invalid_extension() -> anyhow::Result<()> {
    let input_path = "test_input.txt";
    fs::write(input_path, "dummy content")?;

    let status = Command::new("cargo")
        .args(&["run", "--", input_path])
        .status()?;

    assert!(!status.success());

    fs::remove_file(input_path)?;
    Ok(())
}
