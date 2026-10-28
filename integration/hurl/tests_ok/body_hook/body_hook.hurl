# The request body contains dynamic values. It is signed by the `--body-hook` command
# once rendered, and the signature is appended as a trailer.
POST http://localhost:8000/body-hook
Content-Type: application/xml
file,body_hook.xml;
replaceRegex /<SaleId>.*<\/SaleId>/ "<SaleId>sal_{{randomString 28}}</SaleId>"
replace "EUR" "{{currency}}"
HTTP 200


# Requests without body are not passed to the hook.
GET http://localhost:8000/body-hook
HTTP 200
