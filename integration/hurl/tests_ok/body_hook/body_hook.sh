#!/bin/bash
set -Eeuo pipefail

hurl --variable currency=GBP --body-hook "python3 tests_ok/body_hook/body_hook_sign.py" tests_ok/body_hook/body_hook.hurl
