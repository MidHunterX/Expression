get_current_time() {
  date +"%H:%M"
}

# TODO: Recalibration logic
calculate_recalibrate_mins() {
  local interval="$1"
  local minute=$(date +"%M")
  # 10# == "base 10"; because "0.*" == "octal"
  echo $((interval - (10#$minute % interval)))
}

calculate_next_update_time() {
  local current_time="$1"
  local interval_minutes="$2"

  local hour=$(echo "$current_time" | cut -d':' -f1)
  local minute=$(echo "$current_time" | cut -d':' -f2)

  local next_minute=$((($minute / $interval_minutes + 1) * $interval_minutes))
  local next_hour=$hour

  # Handle overflow to next hour
  if [[ $next_minute -ge 60 ]]; then
    next_minute=$((next_minute - 60))
    next_hour=$(((10#$next_hour + 1) % 24))
  fi

  printf "%02d:%02d" $next_hour $next_minute
}

calculate_wait_seconds() {
  local target_time="$1"
  local current_time=$(date +"%H:%M")

  local target_seconds=$(($(date -d "today $target_time" +%s) % 86400))
  local current_seconds=$(($(date -d "today $current_time" +%s) % 86400))

  local wait_seconds=$((target_seconds - current_seconds))

  if [[ $wait_seconds -lt 0 ]]; then
    wait_seconds=$((wait_seconds + 86400))
  fi

  echo $wait_seconds
}
