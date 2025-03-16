#!/usr/bin/env bash

# ------------------------------------------------------------------------------
# Constants and initialization
# ------------------------------------------------------------------------------

# Get script directory (works with symlinks)
SCRIPT_DIR="$(cd "$(dirname "$(readlink -f "${BASH_SOURCE[0]}")")" && pwd)"
cd "$SCRIPT_DIR"

source ./config.conf
source ./utils/time.sh
source ./utils/logger.sh
source ./backends/main.sh
source ./core/engine.sh

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

# todo: Get all wallpapers
# wallpapers > root(special | vibe | nsfw | ...)
# root > main_entry.jpg
# main_entry > sub_entry.jpg

# while true; do
  current_time=$(get_current_time)
  log_info "Current time: $current_time"

  # todo: REFRESH_INTERVAL logic for spacing out
  # REFRESH_INTERVAL = 60 if 24 hour cycle
  # REFRESH_INTERVAL = 24/n(images)*60 if spaceout
  # REFRESH_INTERVAL = {user_input} if random

  next_update_time=$(calculate_next_update_time "$current_time" "${NEXT_UPDATE_INTERVAL:-60}")
  log_info "Next update time: $next_update_time"

  wallpaper_path=$(select_wallpaper "$current_time")
  log_info "Selected wallpaper: $wallpaper_path"

  apply_wallpaper_backend "$wallpaper_path"

  # todo: Internal Refresh calibration
  # if REFRESH_INTERVAL < 60 CALIBRATE_INTERVAL = n/4
  # if REFRESH_INTERVAL < 30 CALIBRATE_INTERVAL = n/2

# done
