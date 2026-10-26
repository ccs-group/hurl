Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --variable currency=GBP --body-hook "python tests_ok/body_hook/body_hook_sign.py" tests_ok/body_hook/body_hook.hurl
