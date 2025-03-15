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

# "Check if script is already running"; $0 == filename; $$ == PID
if [[ $(pgrep -f $0) != "$$" ]]; then
  log_info "Another instance is already running"
  log_info "Exiting"
  exit 0
fi

# ------------------------------------------------------------------------------
# Main loop
# ------------------------------------------------------------------------------

log_info "Starting Expression Wallpaper Manager"
initialize_backend
