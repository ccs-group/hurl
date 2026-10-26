# Replace parts of a file body before sending it.
POST http://localhost:8000/post-file-replace
Content-Type: application/xml
file,post_file_replace.xml;
replaceRegex /<SaleId>.*<\/SaleId>/ "<SaleId>sal_{{randomString 28}}</SaleId>"
# Comments are allowed between replacements
replaceRegex /<Name>.*<\/Name>/ "<Name>{{randomFirstName}}</Name>"
replace "EUR" "{{currency}}"
HTTP 200
