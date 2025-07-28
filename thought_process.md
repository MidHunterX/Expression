### Future Features

- [x] Execute custom scripts on wallpaper change
- [ ] Time-recalibration for system hibernation/sleep recovery
- [ ] Interruptible Sleep Cycle
- [ ] Randomization Scopes:
  - [x] Within Current Group
  - [ ] Within Current Collection
  - [ ] Across all Collections (excluding Special)
  - [ ] Across all Wallpapers (excluding Special)
- [ ] Spread collection of wallpapers throughout the day
- [x] Spread group of wallpapers throughout an hour
- [ ] Efficient, event-driven execution (no polling or sleeps)
- [ ] Interactive Terminal User Interface (TUI) / Slint UI Configurator
- [ ] Inter Process Communication (IPC) support for external control or dynamic updates
- [ ] Expression kill / restart IPC
- [ ] Collection switching API
  - Root-level: Collections (e.g., Wallpapers, Catppuccin, Gruvbox, Not_SFW, Dark_Mode)
  - Mid-level: Time-based Wallpaper Items (Entries and Groups)
  - Leaf-level: Wallpapers (image files)


```toml
[backend.custom]
cmd_check_availability = "backend --version"
cmd_wallpaper_change = "backend set <img_arguments>"
supported_extensions = ["jpg", "jpeg", "png", "gif"]

[backend.swww]
post_command = "~/.config/waypaper/post_wallpaper_change.sh $wallpaper"
img_arguments = "--transition-type fade"
```

## Project Requirements

### Core Features

- [x] 1. Multi-backend support (swww, feh, etc.)
- [x] 2. 24-hour wallpaper cycling with dedicated wallpaper for each hour
- [x] 3. Priority-based wallpaper overrides
- [ ] 4. Collection management for organizing wallpapers by theme
- [ ] 5. Collection flow control (time-based collection switching)
- [ ] 6. IPC interface for programmatic wallpaper control
- [x] 7. Sub-collections for optional wallpaper sets within each hour
- [ ] 8. Custom script execution support (notifications, pywal integration, etc.)
- [ ] 9. Randomization with configurable scopes
- [ ] 10. Per-collection configuration
- [ ] 11. Time recalibration for system hibernation/sleep recovery
- [ ] 12. Wallpaper Sourcing Strategies
  - [x] 12.1. Special Collection Strategy
  - [x] 12.2. Time-based Collection Strategy
  - [ ] 12.3. Theme-based Collection Strategy
  - [ ] 12.4. Root Strategy (Wallpaper Dir)
  - [ ] 12.5. Randomized Scope Strategy
- [ ] 13. Wallpaper Application Algorithms
  - [x] 13.1. 24-hour cycle: Fixed time based on filename
  - [x] 13.2. Spread out (ordered | random): n/m (n = number of wallpapers, m = number of hours)
  - [ ] 13.3. Randomized Scope (Within Collection)

## Design Considerations

### Pseudocode

```c
// Backend Initialization
backend = get_backend("backend_name");
backend.init() {
  check_if_available();
  initialize();
}

// Wallpaper Sourcing
wallpaper_dir = config.get("wallpaper_dir"); // Default
special_dir = config.get("special_dir"); // User-specified
collection_dir = config.get("collection_dir"); // User-specified

if (wallpapers.empty()) {
  exit();
}

// Wallpaper Selection
String select_wallpaper() {
  hour = get_current_time.hour();

  // Special Collection - High-priority
  special_wallpaper = find_wallpaper_in(special_dir, hour);
  if (special_wallpaper.exists()) return special_wallpaper;

  // Randomization
  random_wallpaper = get_random_wallpaper();
  if (random_wallpaper.exists()) return random_wallpaper;

  // Time-based Collection - User specified
  collection_wallpaper = find_wallpaper_in(collection_dir, hour);
  if (collection_wallpaper.exists()) return collection_wallpaper;

  // Time-based Default
  wallpaper = find_wallpaper_in(wallpaper_dir, hour);
  if (wallpaper.exists()) return wallpaper;

  return null;
}

backend.apply_wallpaper("wallpaper_path");

// HELPER FUNCTIONS

String find_wallpaper_in(dir, hour) {
  entry = dir.get(hour);
  if (!entry.exists()) return "";

  // sub-collection
  if (entry.is_dir()) {
    if (randomization.scope == "sub-collection") {
      return entry.get_random_file();
    } else {
      return entry[0]; // first file
    }
  }
  // wallpaper file
  return entry;
}

String get_random_wallpaper() {
  if (randomization.scope == "all") {
    return wallpapers.get_random_file();
  }
  if (randomization.scope == "collection") {
    return collection_dir.get_random_file();
  }
  return "";
}
```

### Priority and Collections

The system will implement a hierarchical approach to wallpaper selection:

1. **Collections**: Logical groups of wallpapers sharing a common theme (work, gaming, dark mode, etc.)
2. **Priority Levels**:
   - High-priority override directories
   - Standard 24-hour cycle directories
   - Default fallback wallpapers
3. **Collection Flow**: Automatic switching between collections based on time of day
   - Example: Light wallpapers during day, dark wallpapers at night

### Randomization

Wallpaper randomization will be implemented with configurable scopes:

- Within a specific hour's sub-collection
- Across an entire collection
- Across all available wallpapers

### Time Management

The system will handle time-related challenges:

- Recalibration after system sleep/hibernation
- Scheduling next wallpaper change
- Time-based collection transitions

## Dataflow

Initialization -> Decision -> Execution -> Waiting

### Initialization

- Load system configuration
- Initialize logging system
- Initialize backend interface

### Decision

- If special collection is active, use it
- Else If time-based collection is active, use it
  - If sub-collection is available, use it
  - Else use default collection
- Else skip

## Implementation Strategy

### Backend Architecture

- Standardized backend interface with consistent I/O format
- Each backend implementation must provide:
  - Initialization function (availability check, setup)
  - Wallpaper application function
  - Configuration options
  - Supported extensions

### Wallpaper Application Algorithms

- 24-hour cycle: Fixed time based on filename
- Spread out (ordered | random): n/m (n = number of wallpapers, m = number of hours)
  - Special Collection could conflict with this due to inconsistent timings
  - Would be problematic on large collections

### Sub-collection Structure

- Directory-based organization following the pattern:
  ```
  wallpaper_dir/
  ├── 0000.jpg      # Standard wallpaper for midnight (00:00)
  ├── 0500.jpg      # Standard wallpaper for 5:00
  ├── 2100.jpg      # Standard wallpaper for 21:00
  ├── 2100/         # Sub-collection for 21:00
  │   ├── E.jpg     # Alternative wallpapers
  │   ├── Zucc.jpg  # for midnight hour
  │   └── ...
  ├── 2200.jpg      # Standard wallpaper for 1:00
  └── ...
  ```
  - Follow this pattern for multiple collections:
  ```
  wallpaper_dir/
  │   special/          # High-priority override directory
  │   └── 0500.jpg      # Wallpaper override for 5:00
  │   collection1/      # Standard wallpaper collection
  │   └── ...
  │   collection2/
  │   ├── 0000.jpg      # Standard wallpaper for midnight (00:00)
  │   ├── 0000/         # Sub-collection for midnight
  │   │   ├── E.jpg     # Alternative wallpapers
  │   │   ├── Zucc.jpg  # for midnight hour
  │   │   └── ...
  │   ├── 0100.jpg      # Standard wallpaper for 1:00
  │   └── ...
  └── ...
  ```
- Sub-collections can be used in both standard and high-priority directories
- Dir names with `HHMM` format are treated as sub-collections
- Dir names with any other format are treated as collections

### Collection Flow

- This can be achieved via cli commands to switch between collections.

### Configuration System

- Hierarchical configuration:
  - System-wide defaults
  - Per-collection settings
  - Time-specific overrides
- Support for external script integration
- Backend-specific parameters

### IPC Interface

- Socket-based communication protocol
- Command API for:
  - Immediate wallpaper changes
  - Collection switching
  - Configuration updates
  - Status queries
