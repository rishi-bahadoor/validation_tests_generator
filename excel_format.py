import sys
from openpyxl import load_workbook
from openpyxl.worksheet.datavalidation import DataValidation

def format_excel_sheet(path):
    wb = load_workbook(path)
    ws = wb.active

    # Create dropdown for Status column (assumed to be column F)
    dv = DataValidation(type="list", formula1='"Pass,Fail,N/A"', allow_blank=True)
    ws.add_data_validation(dv)

    # Apply to rows 6 through 100
    for row in range(6, 101):
        dv.add(ws[f"F{row}"])

    wb.save(path)
    print(f"✅ Excel formatting applied to: {path}")

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python format_excel.py <path_to_excel_file>")
        sys.exit(1)

    excel_path = sys.argv[1]
    format_excel_sheet(excel_path)