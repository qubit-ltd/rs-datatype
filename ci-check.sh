#!/bin/bash
set -euo pipefail

PROJECT_ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
exec env \
  RS_CI_PROJECT_ROOT="$PROJECT_ROOT" \
  RS_CI_FUZZ_MAX_LEN="${RS_CI_FUZZ_MAX_LEN:-16384}" \
  "$PROJECT_ROOT/.rs-ci/ci-check.sh" "$@"
