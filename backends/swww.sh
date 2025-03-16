initialize() {
  log_info "Initializing SWWW backend"

  # Check if SWWW is installed
  if ! command -v swww &>/dev/null; then
    log_error "SWWW is not installed"
    return 1
  fi

  # Check if SWWW daemon is running
  for ((i = 0; i < 5; i++)); do
    if swww query &>/dev/null; then
      log_info "SWWW daemon is already running"
      return 0
    else
      log_debug "SWWW daemon not detected, retrying..."
      sleep 1
    fi
  done

  # Try to start SWWW daemon if not running
  log_warning "SWWW daemon not running, attempting to start"
  if command -v swww &>/dev/null; then
    swww init
    sleep 1

    if swww query &>/dev/null; then
      log_info "SWWW daemon started successfully"
      return 0
    else
      log_error "Failed to start SWWW daemon"
      return 1
    fi
  else
    log_error "SWWW is not installed"
    return 1
  fi
}

apply_wallpaper() {
  local wallpaper_path="$1"

  # Ensure SWWW daemon is running
  if ! swww query &>/dev/null; then
    log_warning "SWWW daemon not running, reinitializing"
    initialize
  fi

  log_debug "Applying wallpaper with SWWW: $wallpaper_path"
  local cmd="swww img $wallpaper_path -t center"

  eval "$cmd"

  return $?
}
