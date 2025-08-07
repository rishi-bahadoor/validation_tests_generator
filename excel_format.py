import sys
from openpyxl import load_workbook
from openpyxl.utils import get_column_letter
from openpyxl.worksheet.datavalidation import DataValidation
from openpyxl.styles import Alignment, Font, PatternFill
from openpyxl.formatting.rule import FormulaRule

STATUS_OPTIONS = ["Pass", "Fail", "Pending", "Blocked"]

STATUS_COLORS = {
    "Pass": "00C851",     # Green
    "Fail": "ff4444",     # Red
    "Pending": "ffbb33",  # Orange
    "Blocked": "33b5e5"   # Blue
}

def apply_conditional_formatting(ws, header_name, rules, max_search_rows=10):
    column_letter = find_column_letter(ws, header_name, max_search_rows)
    row_start = find_row(ws, header_name, max_search_rows) + 1
    row_end = row_start + 94

    for value, hex_color in rules.items():
        fill = PatternFill(start_color=hex_color, end_color=hex_color, fill_type="solid")
        rule = FormulaRule(formula=[f'${column_letter}{row_start}="{value}"'], fill=fill)
        ws.conditional_formatting.add(f"{column_letter}{row_start}:{column_letter}{row_end}", rule)

def bold_cell(ws, header_name, max_search_rows=10):
    column_letter = find_column_letter(ws, header_name, max_search_rows)
    row_number = find_row(ws, header_name, max_search_rows)
    cell = ws[f"{column_letter}{row_number}"]
    cell.font = Font(bold=True)


def wrap_all_cells(ws):
    wrap_alignment = Alignment(wrap_text=True)
    for row in ws.iter_rows():
        for cell in row:
            cell.alignment = wrap_alignment

def find_column_letter(ws, header_name, max_search_rows):
    for row in range(1, max_search_rows + 1):
        for col in range(1, ws.max_column + 1):
            cell_value = ws.cell(row=row, column=col).value
            if cell_value and cell_value.strip().lower() == header_name.strip().lower():
                return get_column_letter(col)
    raise ValueError(f"Header '{header_name}' not found in first {max_search_rows} rows")

def find_row(ws, header_name, max_search_rows):
    for row in range(1, max_search_rows + 1):
        for col in range(1, ws.max_column + 1):
            cell_value = ws.cell(row=row, column=col).value
            if cell_value and cell_value.strip().lower() == header_name.strip().lower():
                return row
    raise ValueError(f"Header '{header_name}' not found in first {max_search_rows} rows")

def add_drop_down(ws, column_letter,options, row_start, row_end):
    dv = DataValidation(type="list", formula1=f'"{options}"', allow_blank=True)
    ws.add_data_validation(dv)

    for row in range(row_start, row_end + 1):
        dv.add(ws[f"{column_letter}{row}"])

def set_column_width(ws, header_name, width, max_search_rows=10):
    column_letter = find_column_letter(ws, header_name, max_search_rows)
    ws.column_dimensions[column_letter].width = width

def set_drop_down(ws, header_name, options, max_search_rows=10):
    column_letter = find_column_letter(ws, header_name, max_search_rows)
    row_start = find_row(ws, header_name, max_search_rows) + 1
    row_end = row_start + 94
    add_drop_down(ws, column_letter, options, row_start, row_end)

def format_excel_sheet(path):
    wb = load_workbook(path)
    ws = wb.active

    set_drop_down(ws, "Status", ",".join(STATUS_OPTIONS))
    apply_conditional_formatting(ws, "Status", STATUS_COLORS)

    set_column_width(ws, "Test ID", 22)
    set_column_width(ws, "Test Group", 18)
    set_column_width(ws, "Priority", 10)
    set_column_width(ws, "Description", 90)
    set_column_width(ws, "Pass Condition", 30)
    set_column_width(ws, "Notes", 50)

    bold_cell(ws, "Test ID")
    bold_cell(ws, "Test Group")
    bold_cell(ws, "Priority")
    bold_cell(ws, "Description")
    bold_cell(ws, "Pass Condition")
    bold_cell(ws, "Status")
    bold_cell(ws, "Notes")

    wrap_all_cells(ws)

    wb.save(path)
    print(f"✅ Excel formatting applied to: {path}")

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python format_excel.py <path_to_excel_file>")
        sys.exit(1)

    excel_path = sys.argv[1]
    format_excel_sheet(excel_path)