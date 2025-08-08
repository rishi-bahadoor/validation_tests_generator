#!/usr/bin/env python3

import sys
import os
from openpyxl import load_workbook
from openpyxl.utils import get_column_letter
from openpyxl.styles import PatternFill
from email.message import EmailMessage
from email import policy

#==============================================================================
# Configuration
#==============================================================================

DEFAULT_XLSX    = "test_report.xlsx"
EML_OUTPUT      = "test_report_email.eml"

EMAIL_FROM_DEFAULT     = "noreply@example.com"
EMAIL_TO_DEFAULT       = "qa-team@example.com"
EMAIL_CC        = ""    # comma-separated; empty = no CC

EMAIL_SUBJECT   = "Validation Report"

# Your exact data headers (used to detect the main table)
HEADERS = [
    "Test ID",
    "Test Group",
    "Priority",
    "Description",
    "Pass Condition",
    "Status",
    "Notes"
]

# Re-use your Excel conditional-formatting colours
STATUS_COLORS = {
    "Pass":    "00C851",   # Green
    "Fail":    "ff4444",   # Red
    "Pending": "ffbb33",   # Orange
    "Blocked": "33b5e5",   # Blue
}

# Sheets that don’t have the above headers and should be rendered generically
GENERIC_SHEETS = {"Technician_Issues", "Github_Issues"}

#==============================================================================
# Helpers
#==============================================================================

def normalize(val):
    return val.strip().lower() if isinstance(val, str) else ""

def find_header_row(ws):
    """Return the row index (1-based) where all HEADERS appear, or None."""
    for r in range(1, min(ws.max_row, 50) + 1):
        row_vals = [normalize(ws.cell(r, c).value) 
                    for c in range(1, ws.max_column + 1)]
        if all(normalize(h) in row_vals for h in HEADERS):
            return r
    return None

def row_is_blank(row):
    return all(cell.value in (None, "") for cell in row)

def style_attrs(cell):
    """
    Inline CSS for:
     - bold font
     - wrap_text
     - Status fills from STATUS_COLORS
    """
    css = []

    if cell.font and cell.font.bold:
        css.append("font-weight:bold;")
    if cell.alignment and cell.alignment.wrap_text:
        css.append("white-space:pre-wrap;")

    # Apply color based on the cell.value → STATUS_COLORS mapping
    v = cell.value
    if isinstance(v, str) and v in STATUS_COLORS:
        css.append(f"background-color:#{STATUS_COLORS[v]};")

    return f' style="{"".join(css)}"' if css else ""

def column_widths(ws, n_cols):
    """
    Return a list of pixel widths for the first n_cols columns,
    approximating Excel's width * 7 + 5px padding.
    """
    out = []
    for c in range(1, n_cols + 1):
        letter = get_column_letter(c)
        w = ws.column_dimensions[letter].width or 8.43
        px = int(w * 7 + 5)
        out.append(px)
    return out

def render_data_sheet(ws):
    """
    Render sheets that contain your main table headers.
    Returns a single HTML snippet for that sheet.
    """
    hdr = find_header_row(ws)
    if hdr is None:
        return None

    # Collect metadata rows (above the header), skipping blank rows
    meta = []
    for r in range(1, hdr):
        row = list(ws[r])
        if not row_is_blank(row):
            meta.append(row)

    # Collect data rows (header + below), skipping blank rows
    data = []
    for r in range(hdr, ws.max_row + 1):
        row = list(ws[r])
        if not row_is_blank(row):
            data.append(row)

    if not data:
        return None

    n_cols = len(data[0])
    widths = column_widths(ws, n_cols)

    html = []

    # Metadata block
    if meta:
        html.append('<table border="0" cellpadding="4" cellspacing="0">')
        for row in meta:
            html.append("<tr>")
            # assume label/value pairs are in cols A+B
            for cell in row[:2]:
                text = cell.value or ""
                html.append(f'<td{style_attrs(cell)}>{text}</td>')
            html.append("</tr>")
        html.append("</table><br/>")

    # Data table with column widths
    html.append('<table border="1" cellpadding="4" cellspacing="0">')
    html.append("<colgroup>")
    for w in widths:
        html.append(f'<col style="width:{w}px;"/>')
    html.append("</colgroup>")

    # Header row
    html.append("<thead><tr>")
    for cell in data[0]:
        text = cell.value or ""
        html.append(f'<th{style_attrs(cell)}>{text}</th>')
    html.append("</tr></thead>")

    # Body rows
    html.append("<tbody>")
    for row in data[1:]:
        html.append("<tr>")
        for cell in row:
            text = cell.value or ""
            html.append(f'<td{style_attrs(cell)}>{text}</td>')
        html.append("</tr>")
    html.append("</tbody></table><br/>")

    return "".join(html)

def render_generic_sheet(ws):
    """
    Render sheets like Technician_Issues or Github_Issues 
    as a simple table of every non-blank row.
    """
    rows = [row for row in ws.iter_rows() if not row_is_blank(row)]
    if not rows:
        return None

    html = ['<table border="0" cellpadding="4" cellspacing="0">']
    for row in rows:
        html.append("<tr>")
        for cell in row:
            if cell.value is not None:
                text = cell.value
                html.append(f'<td{style_attrs(cell)}>{text}</td>')
        html.append("</tr>")
    html.append("</table><br/>")

    return "".join(html)

def extract_tables(path):
    wb = load_workbook(path, data_only=False)
    result = []

    for ws in wb.worksheets:
        # 1) Main data sheets
        snippet = render_data_sheet(ws)
        if snippet:
            result.append(snippet)
            continue

        # 2) Generic sheets
        if ws.title in GENERIC_SHEETS:
            snippet = render_generic_sheet(ws)
            if snippet:
                result.append(snippet)

    return result

#==============================================================================
# Email assembly
#==============================================================================

def build_body(tables):
    html = ["<html><body>"]
    html.extend(tables)
    html.append("</body></html>")
    return "".join(html)

def make_email(html, from_addr, to_addr):
    msg = EmailMessage(policy=policy.SMTP)
    msg["Subject"] = EMAIL_SUBJECT
    msg["From"]    = from_addr
    msg["To"]      = to_addr
    if EMAIL_CC.strip():
        msg["Cc"] = EMAIL_CC

    msg.set_content("This report is best viewed in an HTML email client.")
    msg.add_alternative(html, subtype="html")
    return msg

def save_eml(msg, path):
    with open(path, "wb") as f:
        f.write(msg.as_bytes())

#==============================================================================
# Main
#==============================================================================

def main():
    # if two args passed → override defaults
    if len(sys.argv) == 3:
        from_addr = sys.argv[1]
        to_addr   = sys.argv[2]
    elif len(sys.argv) == 1:
        from_addr = EMAIL_FROM_DEFAULT
        to_addr   = EMAIL_TO_DEFAULT
    else:
        print(f"Usage: {sys.argv[0]} [sender_email recipient_email]")
        sys.exit(1)

    if not os.path.exists(DEFAULT_XLSX):
        print(f"Error: `{DEFAULT_XLSX}` not found.")
        sys.exit(1)

    tables = extract_tables(DEFAULT_XLSX)
    body   = build_body(tables)
    msg    = make_email(body, from_addr, to_addr)
    save_eml(msg, EML_OUTPUT)
    print(f"✅ Generated `{EML_OUTPUT}`. Verify in Outlook.")


if __name__ == "__main__":
    main()