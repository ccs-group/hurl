#!/bin/bash
set -Eeuo pipefail

hurl --variable currency=GBP tests_ok/post/post_file_replace.hurl
