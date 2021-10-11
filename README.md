# bwexport
Exporting Bitwarden Org Items From Normal User

1. This is not an official script from Bitwarden
2. Please verify the exported data. Author does not responsible for any discrepancies
3. Only includes Login & Note items. Does not include Identity, Card items

v0.1:
- Collections are not included
- Custom Fields are not included

To-Do:
- Include Custom Fields
- Include Collections
- Error Handling
- Code Cleaning & Optimization
- JSON Export

How To Use:
- Download bwexport.exe & bw.exe (Bitwarden CLI), put them in the same directory
- login using bw.exe (https://bitwarden.com/help/article/cli/)
- Copy the session key
- run bwexport.exe, enter the session key
- CSV file (Export_Org_Items.csv) will be exported in the same directory