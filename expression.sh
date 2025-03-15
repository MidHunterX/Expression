#!/usr/bin/env bash

# Strict mode
set -euo pipefail

# ------------------------------------------------------------------------------
# Constants and initialization
# ------------------------------------------------------------------------------

# Get script directory (works with symlinks)
SCRIPT_DIR="$(cd "$(dirname "$(readlink -f "${BASH_SOURCE[0]}")")" && pwd)"
cd "$SCRIPT_DIR"

source ./config.conf
source ./utils/logger.sh
source ./backends/main.sh

# ------------------------------------------------------------------------------
# Main loop
# ------------------------------------------------------------------------------

initialize_logger
log_info "Starting Expression Wallpaper Manager"
initialize_backend
