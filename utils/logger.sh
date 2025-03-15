initialize_logger() {
  LOG_LEVEL="${LOG_LEVEL:-INFO}"
  LOG_TO_CONSOLE="${LOG_TO_CONSOLE:-false}"
}

log_message() {
  local level="$1"
  local message="$2"
  local timestamp=$(date +"%Y-%m-%d %H:%M:%S")

  # Log levels: DEBUG=0, INFO=1, WARNING=2, ERROR=3
  local level_num=1
  case "$level" in
  "DEBUG") level_num=0 ;;
  "INFO") level_num=1 ;;
  "WARNING") level_num=2 ;;
  "ERROR") level_num=3 ;;
  esac

  # Current log level
  local current_level_num=1
  case "$LOG_LEVEL" in
  "DEBUG") current_level_num=0 ;;
  "INFO") current_level_num=1 ;;
  "WARNING") current_level_num=2 ;;
  "ERROR") current_level_num=3 ;;
  esac

  if [[ $level_num -ge $current_level_num ]]; then
    if [[ "$LOG_TO_CONSOLE" == "true" ]]; then
      local color_code=""
      case "$level" in
      "DEBUG") color_code="\033[36m" ;;   # Cyan
      "INFO") color_code="\033[32m" ;;    # Green
      "WARNING") color_code="\033[33m" ;; # Yellow
      "ERROR") color_code="\033[31m" ;;   # Red
      esac
      echo -e "${color_code}[$level] $message\033[0m"
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
