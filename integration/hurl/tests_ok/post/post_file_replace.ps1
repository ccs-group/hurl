Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --variable currency=GBP tests_ok/post/post_file_replace.hurl
