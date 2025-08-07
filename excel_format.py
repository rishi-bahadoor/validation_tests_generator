import sys
from openpyxl import load_workbook
from openpyxl.worksheet.datavalidation import DataValidation
from openpyxl.utils import get_column_letter

def find_column_letter(ws, header_name, max_search_rows):
    for row in range(1, max_search_rows + 1):
        for col in range(1, ws.max_column + 1):
            cell_value = ws.cell(row=row, column=col).value
            if cell_value and cell_value.strip().lower() == header_name.strip().lower():
                return get_column_letter(col)
    raise ValueError(f"Header '{header_name}' not found in first {max_search_rows} rows")


def add_drop_down(ws, column_letter,options, row_start, row_end):
    dv = DataValidation(type="list", formula1=f'"{options}"', allow_blank=True)
    ws.add_data_validation(dv)

    for row in range(row_start, row_end + 1):
        dv.add(ws[f"{column_letter}{row}"])

def format_excel_sheet(path):
    wb = load_workbook(path)
    ws = wb.active

    column_letter = find_column_letter(ws, "Status",10)
    add_drop_down(ws, column_letter, "Pass,Fail,Pending,Blocked", 6, 100)

    wb.save(path)
    print(f"✅ Excel formatting applied to: {path}")

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python format_excel.py <path_to_excel_file>")
        sys.exit(1)

    excel_path = sys.argv[1]
    format_excel_sheet(excel_path)