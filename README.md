# Expression

## Requirements

- [ ] 1. Change wallpapers with any backends. Like swww, feh etc.
- [ ] 2. 24 hour wallpaper cycling. Dedicated wallpaper for each hour.
- [ ] 3. Higher priority special wallpapers which overrides normal cycle.
- [ ] 4. Collections
- [ ] 5. Collection Flow control
- [ ] 6. Interface for other process to change wallpaper programmatically (IPC)
- [ ] 7. Sub collections - optional set of wallpapers for each hour
- [ ] 8. Custom script execution support - for notification, pywal etc.
- [ ] 9. Random wallpapers
- [ ] 10. Per collection config

- [ ] Debug Logging
- [ ] Detailed Testing

### Though process

Requirement 3 & 4: High priority wallpapers and collections

- Currently this feature is being implemented via setting env variables in format `T_HHMM`.
- Then checking if that variable is set else follow lower priority (24 hour cycling)
- A better way would be setting high priority dir (override) and low priority dir (24 hour cycle).
- Maybe even scale it to multiple low priority dirs (collections).
- Collections can be a collection of wallpapers for different vibes (gaming, work, dark_mode, nsfw etc).

Requirement 5: Collection Flow Control

- Seeing different collections, it would be nice to set different collections at different time of the day.
- Like light wallpapers during the day and dark wallpapers during the night.

Requirement 9: Random Wallpapers

- Randomization with different scopes like all, subcollection etc.
- Randomization config for each collection

## Implementation

### Requirement 1: Switchable Backends

- Each backend entry needs to have a standardized io format.
- Provide function for:
  - Initializing backend - checking if available and setting up.
  - Setting wallpaper
  - additional arguments

### Requirement 7: Sub Collections

- Add an optional dir with the name of hour to enable random wallpaper feature for that hour.
- Sub collection feature can be used on both high and low priority dirs.

## Behaviors

- If backend is not set up, this script will initialize it for you and run rest of the script anyway.
