# Expression

Express vibes with wallpapers

## Project Requirements

### Core Features

- [ ] 1. Multi-backend support (swww, feh, etc.)
- [ ] 2. 24-hour wallpaper cycling with dedicated wallpaper for each hour
- [ ] 3. Priority-based wallpaper overrides
- [ ] 4. Collection management for organizing wallpapers by theme
- [ ] 5. Collection flow control (time-based collection switching)
- [ ] 6. IPC interface for programmatic wallpaper control
- [ ] 7. Sub-collections for optional wallpaper sets within each hour
- [ ] 8. Custom script execution support (notifications, pywal integration, etc.)
- [ ] 9. Randomization with configurable scopes
- [ ] 10. Per-collection configuration
- [ ] 11. Time recalibration for system hibernation/sleep recovery
- [ ] 12. Comprehensive logging system
- [ ] 13. Extensive test coverage

## Design Considerations

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

### Waiting Recalculation Strategy (Scheduling)

Accuracy: 1  # seconds
- (T x 60) / 2
- 4hr: T/2 - 2hr
- 2hr: T/2 - 1hr
- 1hr: T/2 - 30min
- 30min: T/2 - 15min
- 15min: T/2 - 7.5min
- 7.5min: T/2 - 3.75min
- 3.75min: T/2 - 1.875min
- 1.875min: T/2 - 0.9375min
- Accuracy <= 1 - skip
- ~0.9375min: T/2 - 0.46875min~
- ~0.46875min: T/2 - 0.234375min~
- ~0.234375min: T/2 - 0.1171875min~
- ~0.1171875min: T/2 - 0.05859375min~
- ~0.05859375min: T/2 - 0.029296875min~

### Sub-collection Structure

- Directory-based organization following the pattern:
  ```
  collection/
  ├── 00.jpg        # Standard wallpaper for midnight (00:00)
  ├── 00/           # Sub-collection for midnight
  │   ├── E.jpg     # Alternative wallpapers
  │   ├── Zucc.jpg  # for midnight hour
  │   └── ...
  ├── 01.jpg        # Standard wallpaper for 1:00
  └── ...
  ```
- Sub-collections can be used in both standard and high-priority directories

### Collection Flow

- Out of scope. This can be achieved via cli commands to switch between collections.

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
