initialize() {
  log_info "Initializing Feh backend"

  # Check if Feh is installed
  if ! command -v feh &>/dev/null; then
    log_error "Feh is not installed"
    return 1
  fi

  log_info "Feh backend initialized"
  return 0
}

apply_wallpaper() {
  local wallpaper_path="$1"
  log_debug "Applying wallpaper with Feh: $wallpaper_path"
  local cmd="feh --bg-scale \"$wallpaper_path\""
  eval "$cmd"
  return $?
}
