// src/email_ops.rs

use calamine::{DataType, Range, Reader, Xlsx};
use lettre::{Message, message::header::ContentType};
use std::{fs, fs::File, path::Path};

const DEFAULT_XLSX: &str = "test_report.xlsx";

// src/email_ops.rs

//------------------------------------------------------------------------------
// Email address constants
//------------------------------------------------------------------------------

/// Sender address
const EMAIL_FROM: &str = "noreply@example.com";

/// Primary recipient
const EMAIL_TO: &str = "qa-team@example.com";

/// (Optional) CC recipients, comma-separated
const EMAIL_CC: &str = "";

//------------------------------------------------------------------------------
// Non-dependent helper functions
//------------------------------------------------------------------------------
/// Scan the entire sheet range and return (rows, columns) of the used area.
/// Rows and columns are counts, so if the max index is 53, rows = 54.
fn find_used_range(range: &Range<DataType>) -> (usize, usize) {
    let mut max_row = 0;
    let mut max_col = 0;

    // Destructure as (row, col, cell)
    for (r, c, cell) in range.cells() {
        if !cell.is_empty() {
            max_row = max_row.max(r);
            max_col = max_col.max(c);
        }
    }

    (max_row + 1, max_col + 1)
}

/// Turn the upper-left sub‐range (0..rows, 0..cols) of an Excel sheet into an HTML table.
fn range_to_html_table(range: &Range<DataType>, rows: usize, cols: usize) -> String {
    let mut html = String::from(r#"<table border="1" cellpadding="4" cellspacing="0">"#);
    html.push('\n');

    for r in 0..rows {
        html.push_str("  <tr>");
        for c in 0..cols {
            let cell = range.get((r, c)).unwrap_or(&DataType::Empty);
            let text = match cell {
                DataType::String(s) => s.clone(),
                DataType::Float(f) => f.to_string(),
                DataType::Int(i) => i.to_string(),
                DataType::Bool(b) => b.to_string(),
                _ => String::new(),
            };
            html.push_str(&format!("<td>{}</td>", text));
        }
        html.push_str("</tr>\n");
    }

    html.push_str("</table>\n");
    html
}

//------------------------------------------------------------------------------
// Dependent helper functions
//------------------------------------------------------------------------------

/// Open the XLSX at `path`, extract each sheet’s used area,
/// convert it to HTML, and prepend a heading with the sheet name.
fn extract_tables_from_workbook(path: &Path) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // Open a File (implements Read + Seek)
    let file = File::open(path)?;
    let mut workbook: Xlsx<_> = Xlsx::new(file)?;

    let mut tables = Vec::new();
    for sheet_name in workbook.sheet_names().to_owned() {
        if let Some(Ok(range)) = workbook.worksheet_range(&sheet_name) {
            let (rows, cols) = find_used_range(&range);
            let table_html = range_to_html_table(&range, rows, cols);
            tables.push(format!("<h2>{}</h2>\n{}", sheet_name, table_html));
        }
    }

    Ok(tables)
}

/// Wrap all the individual sheet–tables into a single HTML body.
fn build_email_body(tables: Vec<String>) -> String {
    let mut body = String::from("<html><body>\n");
    body.push_str("<h1>Automated Test Report</h1>\n");

    for tbl in tables {
        body.push_str(&tbl);
    }

    body.push_str("</body></html>");
    body
}

//------------------------------------------------------------------------------
// Main "generate_email" function
//------------------------------------------------------------------------------

pub fn generate_email() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(DEFAULT_XLSX);

    if !path.exists() {
        eprintln!("Error: `{}` not found in current directory.", DEFAULT_XLSX);
        return Ok(());
    }

    // Read workbook & build HTML tables
    let tables = extract_tables_from_workbook(path)?;
    let html_body = build_email_body(tables);

    // Construct an .eml file with HTML body
    let mut builder = Message::builder()
        .from(EMAIL_FROM.parse()?)
        .to(EMAIL_TO.parse()?)
        .subject("Validation Test Report")
        .header(ContentType::TEXT_HTML);

    // Add CC addresses from constant
    for addr in EMAIL_CC.split(',') {
        let addr = addr.trim();
        if !addr.is_empty() {
            builder = builder.cc(addr.parse()?);
        }
    }

    // Finalize the email
    let email = builder.body(html_body)?;

    // Serialize to disk
    let raw = email.formatted();
    fs::write("test_report_email.eml", raw)?;
    println!("✅ Generated `test_report_email.eml`. You can open it in Outlook.");

    Ok(())
}
