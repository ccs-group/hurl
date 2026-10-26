#!/bin/bash
set -Eeuo pipefail

hurl --body-hook "echo invalid key >&2; exit 3" tests_failed/body_hook/body_hook.hurl
