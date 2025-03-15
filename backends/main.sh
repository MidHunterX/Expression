initialize_backend() {
  log_info "Initializing backend: $BACKEND"

  case "$BACKEND" in
  "swww")
    source ./backends/swww.sh
    initialize
    ;;

  "feh")
    source ./backends/feh.sh
    initialize
    ;;

  *)
    log_error "Unknown backend: $BACKEND"
    exit 1
    ;;

  esac
}
