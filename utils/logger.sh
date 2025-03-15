initialize_logger() {
  LOG_LEVEL="${LOG_LEVEL:-INFO}"
  LOG_TO_CONSOLE="${LOG_TO_CONSOLE:-false}"
}

log_message() {
  local -A levels=(["DEBUG"]=0 ["INFO"]=1 ["WARNING"]=2 ["ERROR"]=3)
  local level="${1}"
  local message="$2"
  local timestamp=$(date +"%Y-%m-%d %H:%M:%S")

  # Validate level
  [[ -z "${levels[$level]}" ]] && return

  local level_num=${levels[$level]}
  local current_level_num=${levels[${LOG_LEVEL}]:-1}

  # Filter logs based on $LOG_LEVEL
  if ((level_num >= current_level_num)); then
    if [[ "$LOG_TO_CONSOLE" == "true" ]]; then
      local -A colors=(
        ["DEBUG"]="\033[36m"   # Cyan
        ["INFO"]="\033[32m"    # Green
        ["WARNING"]="\033[33m" # Yellow
        ["ERROR"]="\033[31m"   # Red
      )
      echo -e "${colors[$level]}[$level] $message\033[0m"
    fi
  fi
}

log_debug() {
  log_message "DEBUG" "$1"
}

log_info() {
  log_message "INFO" "$1"
}

log_warning() {
  log_message "WARNING" "$1"
}

log_error() {
  log_message "ERROR" "$1"
}
