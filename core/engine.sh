select_wallpaper() {
  local current_time="$1"

  local hour=$(echo "$current_time" | cut -d':' -f1)

  # SPECIAL WALLPAPERS
  local special_name="T_${hour}00"
  local special_value="${!special_name}"
  if [[ -n "$special_value" ]]; then # Ensure variable exists
    local special_wallpaper="$SPECIAL_DIR/$special_value.jpg"
    if [[ -e "$special_wallpaper" ]]; then
      echo "$special_wallpaper"
      return
    fi
  fi

  selected_wallpaper="$WALLPAPER_DIR/$hour.jpg"

  # RANDOMIZED WALLPAPERS
  if [[ "$ENABLE_RANDOMIZATION" == "true" ]]; then
    case "$RANDOMIZATION_SCOPE" in
    "current")
      # Randomize within current hour directory
      local hour_dir="$WALLPAPER_DIR/$hour"
      if [[ -d "$hour_dir" ]]; then
        selected_wallpaper=$(find "$hour_dir" -type f \( -name "*.jpg" -o -name "*.png" \) | shuf -n 1)
      fi
      ;;
    "all")
      # Randomize across all available wallpapers
      selected_wallpaper=$(find "$WALLPAPER_DIR" -type f \( -name "*.jpg" -o -name "*.png" \) | shuf -n 1)
      ;;
    *)
      selected_wallpaper="$WALLPAPER_DIR/$hour.jpg"
      ;;
    esac
  fi

  # WALLPAPER VALIDATION
  if [[ ! -f "$selected_wallpaper" ]]; then

    if [[ -f "$WALLPAPER_DIR/$hour.jpg" ]]; then
      selected_wallpaper="$WALLPAPER_DIR/$hour.jpg"
    else
      echo ""
      return
    fi
  fi

  echo "$selected_wallpaper"
}

apply_wallpaper_backend() {
  local wallpaper_path="$1"
  if [[ ! -f "$wallpaper_path" ]]; then
    return 1
  fi
  apply_wallpaper "$wallpaper_path"
  return 0
}
